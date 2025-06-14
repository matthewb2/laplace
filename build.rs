// build.rs 파일 내용

fn main() {
    // Windows 환경에서만 리소스를 처리하도록 조건부 컴파일 설정
    if cfg!(target_os = "windows") {
        // 'winres' 크레이트를 사용하여 리소스 설정
        let mut res = winres::WindowsResource::new();

        // 아이콘 파일 경로 설정 (프로젝트 루트 디렉토리에 'icon.ico'가 있다고 가정)
        // 실제 아이콘 파일 경로에 맞게 수정해주세요.
        // 예: res.set_icon("assets/my_icon.ico");
        res.set_icon("logo.ico");

        // 리소스 적용 시도
        match res.compile() {
            Ok(_) => println!("Successfully compiled Windows resources."),
            Err(e) => eprintln!("Failed to compile Windows resources: {}", e),
        }
    }
}
