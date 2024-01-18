use std::process::Command;

pub fn run_media_service(with_libos: bool) -> f64 {
    let output = Command::new("bash")
        .arg(if with_libos {
            "scripts/run_media_service.sh"
        } else {
            "scripts/run_media_service_no_libos.sh"
        })
        .arg("--release")
        .arg("2>&1")
        .output()
        .expect("Failed to execute command");

    // 检查命令是否成功执行
    let stdout = if output.status.success() {
        String::from_utf8(output.stderr).expect("Invalid UTF-8")
    } else {
        // 输出错误信息
        let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");
        panic!("{}", stderr)
    };
    // println!("stderr: {}", stdout);

    let total_dur: f64 = stdout
        .lines()
        .find(|line| line.contains("total_dur"))
        .and_then(|line| {
            line.split(' ')
                .collect::<Vec<_>>()
                .last()
                .map(|s| (*s).to_owned())
        })
        .expect("output do not contain total_dur?")
        .parse()
        .expect("bad total_dur format?");

    total_dur
}
