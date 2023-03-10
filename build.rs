use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> io::Result<()> {
    if env::var_os("FNSA_INIT") != Some(OsString::from("1")) {
        println!("cargo:rerun-if-changed=build.rs");
    }

    const TARGET_PROTO_DIR: &[&str; 2] = &["lbm-sdk/proto", "lbm-sdk/third_party/proto"];

    let proto_out = Path::new("src/prost");
    fs::create_dir_all(proto_out)?;

    let mut proto_files = Vec::new();
    visit_dirs(Path::new("lbm-sdk/proto"), &mut |x| {
        if let Some("proto") = x.to_str()?.split('.').collect::<Vec<&str>>().pop() {
            proto_files.push(x);
        }
        Some(())
    })?;
    trace_proto_paths("proto_list.txt", &proto_files)?;

    let mut builder = prost_build::Config::new();
    builder.include_file("_include.rs").out_dir(proto_out);
    builder.compile_protos(&proto_files, TARGET_PROTO_DIR)?;


    // replace words to resolve conflicting implementation
    for (pattern, replacement) in [
        ("enum Validators", "enum Policy"),
        (
            "stake_authorization::Validators",
            "stake_authorization::Policy",
        ),
    ] {
        patch_file(
            proto_out.join("cosmos.staking.v1beta1.rs"),
            &regex::Regex::new(pattern).unwrap(),
            replacement,
        )
        .expect("error patching cosmos.staking.v1beta1.rs");
    }

    // Deprecated: use built-in prost option instead
    // if env::var_os("INCLUDE_REGEN") == Some(OsString::from("1")) {
    //     gen_proto_include("src/prost", "src/proto.rs")?;
    // }

    Ok(())
}

fn visit_dirs<F>(dir: &Path, cb: &mut F) -> io::Result<()>
where
    F: FnMut(PathBuf) -> Option<()>,
{
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(path);
            }
        }
    }
    Ok(())
}

fn trace_proto_paths(logpath: &str, proto_paths: &Vec<PathBuf>) -> io::Result<()> {
    let logpath = Path::new(logpath);
    if logpath.exists() {
        fs::remove_file(logpath)?;
    }
    fs::File::create(logpath)?;

    let mut logfile = fs::OpenOptions::new()
        .append(true)
        .open(logpath)
        .expect("cannot open this log");

    logfile.write("Proto Version: ".as_ref())?;
    let ver = Command::new("sh")
        .arg("-c")
        .arg("git submodule status lbm-sdk")
        .output()
        .unwrap();
    logfile.write(&ver.stdout).expect("log writing failed");
    logfile.write("\n".as_ref())?;
    for fname in proto_paths.iter() {
        logfile
            .write(format!("{}\n", fname.to_str().unwrap()).as_bytes())
            .expect("log writing failed");
    }

    Ok(())
}

fn patch_file(path: impl AsRef<Path>, pattern: &regex::Regex, replacement: &str) -> io::Result<()> {
    let mut contents = fs::read_to_string(&path)?;
    contents = pattern.replace_all(&contents, replacement).to_string();
    fs::write(path, &contents)
}

#[allow(dead_code)]
fn gen_proto_include(proto_path: &str, out: &str) -> io::Result<()> {
    let out = Path::new(out);
    if out.exists() {
        fs::remove_file(out)?;
    }
    fs::File::create(out)?;
    let mut outfile = fs::OpenOptions::new()
        .append(true)
        .open(out)
        .expect("cannot open output file");

    let p = Path::new(proto_path);
    let mut proto_map = HashMap::<String, Vec<String>>::new();
    for entry in fs::read_dir(p)? {
        let fp = entry?.path();
        let fullpaths = fp.to_str().unwrap().split("/").collect::<Vec<&str>>();
        let filepath = fullpaths[1..].join("/");
        proto_map
            .entry(fullpaths[2].split(".").collect::<Vec<&str>>()[0].to_string())
            .and_modify(|v| v.push(filepath.clone()))
            .or_insert(vec![filepath]);
    }

    let space4 = "    ";
    let _pubmod_prefix =
        |space: &str, name: &str| -> String { format!("{}pub mod {} {{\n", space, name) };
    let _pubmod_suffix = |space: &str| -> String { format!("\n{}}}", space) };
    let _include =
        |space: &str, path: &str| -> String { format!("{}include!(\"{}\");", space, path) };

    for (k, v) in proto_map.iter_mut() {
        v.sort();
        let mut tmpl = _pubmod_prefix(&space4.repeat(0), k);
        let mut first = true;
        let mut cont_sig;

        for (i, f) in v.iter().enumerate() {
            let fullname = f
                .split("/")
                .collect::<Vec<&str>>()
                .pop()
                .unwrap()
                .split(".")
                .collect::<Vec<&str>>();
            let tokens = &fullname[..fullname.len() - 1];
            let next_name = if i != (v.len() - 1) {
                v[i + 1]
                    .split("/")
                    .collect::<Vec<&str>>()
                    .pop()
                    .unwrap()
                    .split(".")
                    .collect::<Vec<&str>>()
            } else {
                Vec::new()
            };
            cont_sig =
                if (i != (v.len() - 1)) && !next_name.is_empty() && (next_name[1] == tokens[1]) {
                    true
                } else {
                    false
                };

            let cur_token_len = tokens.len();

            match cur_token_len {
                2 => tmpl.push_str(
                    format!(
                        "{}{}{}\n",
                        _pubmod_prefix(space4, tokens[1]),
                        _include(&space4.repeat(2), f),
                        _pubmod_suffix(space4)
                    )
                    .as_str(),
                ),
                3 => {
                    if cont_sig {
                        if first {
                            tmpl.push_str(
                                format!(
                                    "{}{}{}{}\n",
                                    _pubmod_prefix(space4, tokens[1]),
                                    _pubmod_prefix(&space4.repeat(2), tokens[2]),
                                    _include(&space4.repeat(3), f),
                                    _pubmod_suffix(&space4.repeat(2)),
                                )
                                .as_str(),
                            );
                            first = false;
                        } else {
                            tmpl.push_str(
                                format!(
                                    "{}{}{}\n",
                                    _pubmod_prefix(&space4.repeat(2), tokens[2]),
                                    _include(&space4.repeat(3), f),
                                    _pubmod_suffix(&space4.repeat(2)),
                                )
                                .as_str(),
                            )
                        }
                    } else {
                        if first {
                            tmpl.push_str(
                                format!(
                                    "{}{}{}{}{}\n",
                                    _pubmod_prefix(space4, tokens[1]),
                                    _pubmod_prefix(&space4.repeat(2), tokens[2]),
                                    _include(&space4.repeat(3), f),
                                    _pubmod_suffix(&space4.repeat(2)),
                                    _pubmod_suffix(space4)
                                )
                                .as_str(),
                            );
                        } else {
                            tmpl.push_str(
                                format!(
                                    "{}{}{}{}\n",
                                    _pubmod_prefix(&space4.repeat(2), tokens[2]),
                                    _include(&space4.repeat(3), f),
                                    _pubmod_suffix(&space4.repeat(2)),
                                    _pubmod_suffix(space4)
                                )
                                .as_str(),
                            );
                            first = true;
                        }
                    }
                }
                4 => {
                    if cont_sig {
                        if first {
                            tmpl.push_str(
                                format!(
                                    "{}{}{}{}{}{}\n",
                                    _pubmod_prefix(space4, tokens[1]),
                                    _pubmod_prefix(&space4.repeat(2), tokens[2]),
                                    _pubmod_prefix(&space4.repeat(3), tokens[3]),
                                    _include(&space4.repeat(4), f),
                                    _pubmod_suffix(&space4.repeat(3)),
                                    _pubmod_suffix(&space4.repeat(2)),
                                )
                                .as_str(),
                            );
                            first = false;
                        } else {
                            tmpl.push_str(
                                format!(
                                    "{}{}{}{}{}\n",
                                    _pubmod_prefix(&space4.repeat(2), tokens[2]),
                                    _pubmod_prefix(&space4.repeat(3), tokens[3]),
                                    _include(&space4.repeat(4), f),
                                    _pubmod_suffix(&space4.repeat(3)),
                                    _pubmod_suffix(&space4.repeat(2)),
                                )
                                .as_str(),
                            );
                        }
                    } else {
                        if first {
                            tmpl.push_str(
                                format!(
                                    "{}{}{}{}{}{}{}\n",
                                    _pubmod_prefix(space4, tokens[1]),
                                    _pubmod_prefix(&space4.repeat(2), tokens[2]),
                                    _pubmod_prefix(&space4.repeat(3), tokens[3]),
                                    _include(&space4.repeat(4), f),
                                    _pubmod_suffix(&space4.repeat(3)),
                                    _pubmod_suffix(&space4.repeat(2)),
                                    _pubmod_suffix(space4)
                                )
                                .as_str(),
                            );
                        } else {
                            tmpl.push_str(
                                format!(
                                    "{}{}{}{}{}{}\n",
                                    _pubmod_prefix(&space4.repeat(2), tokens[2]),
                                    _pubmod_prefix(&space4.repeat(3), tokens[3]),
                                    _include(&space4.repeat(4), f),
                                    _pubmod_suffix(&space4.repeat(3)),
                                    _pubmod_suffix(&space4.repeat(2)),
                                    _pubmod_suffix(space4)
                                )
                                .as_str(),
                            );
                            first = true;
                        }
                    }
                }
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::Unsupported,
                        "more than 4 tokens is not supprted. please check it",
                    ));
                }
            }
        }
        tmpl.push_str(format!("}}\n").as_str());
        outfile.write_all(tmpl.as_ref())?;
    }

    Ok(())
}
