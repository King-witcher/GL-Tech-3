#[derive(Debug)]
pub enum SysRequest {
    SetResolution(u32, u32),
    SetFullscreen(bool),
    SetCaptureMouse(bool),
    SetTitle(String),
    SetVSync(bool),
}

pub struct SystemContext {
    requests: Vec<SysRequest>,
    pub(crate) exit: bool,
}

impl SystemContext {
    pub(crate) fn new() -> Self {
        Self {
            requests: Vec::new(),
            exit: false,
        }
    }

    pub fn set_resolution(&mut self, width: u32, height: u32) {
        self.requests.push(SysRequest::SetResolution(width, height));
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.requests.push(SysRequest::SetFullscreen(fullscreen));
    }

    pub fn set_capture_mouse(&mut self, capture: bool) {
        self.requests.push(SysRequest::SetCaptureMouse(capture));
    }

    pub fn set_title(&mut self, title: &str) {
        self.requests.push(SysRequest::SetTitle(title.into()));
    }

    pub fn set_vsync(&mut self, vsync: bool) {
        self.requests.push(SysRequest::SetVSync(vsync));
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub(crate) fn take_requests(&mut self) -> impl Iterator<Item = SysRequest> {
        let a = std::mem::take(&mut self.requests);
        a.into_iter()
    }
}
