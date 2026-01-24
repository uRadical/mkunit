use crate::error::Result;
use crate::systemd::MKUNIT_MARKER;
use handlebars::Handlebars;
use serde::Serialize;

// Embed templates at compile time
const SERVICE_TEMPLATE: &str = include_str!("../../templates/service.unit");
const TIMER_TEMPLATE: &str = include_str!("../../templates/timer.unit");
const PATH_TEMPLATE: &str = include_str!("../../templates/path.unit");
const SOCKET_TEMPLATE: &str = include_str!("../../templates/socket.unit");
const MOUNT_TEMPLATE: &str = include_str!("../../templates/mount.unit");
const TARGET_TEMPLATE: &str = include_str!("../../templates/target.unit");

/// Template registry for rendering unit files
pub struct Templates {
    handlebars: Handlebars<'static>,
}

impl Templates {
    pub fn new() -> Result<Self> {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);
        // Disable HTML escaping - we're generating INI files, not HTML
        handlebars.register_escape_fn(handlebars::no_escape);

        handlebars.register_template_string("service", SERVICE_TEMPLATE)?;
        handlebars.register_template_string("timer", TIMER_TEMPLATE)?;
        handlebars.register_template_string("path", PATH_TEMPLATE)?;
        handlebars.register_template_string("socket", SOCKET_TEMPLATE)?;
        handlebars.register_template_string("mount", MOUNT_TEMPLATE)?;
        handlebars.register_template_string("target", TARGET_TEMPLATE)?;

        Ok(Self { handlebars })
    }

    fn render_with_marker(&self, template: &str, data: &impl Serialize) -> Result<String> {
        let content = self.handlebars.render(template, data)?;
        // Add marker and clean up blank lines
        let cleaned = clean_unit_content(&content);
        Ok(format!("{MKUNIT_MARKER}\n{cleaned}"))
    }

    pub fn render_service(&self, data: &ServiceData) -> Result<String> {
        self.render_with_marker("service", data)
    }

    pub fn render_timer(&self, data: &TimerData) -> Result<String> {
        self.render_with_marker("timer", data)
    }

    pub fn render_path(&self, data: &PathData) -> Result<String> {
        self.render_with_marker("path", data)
    }

    pub fn render_socket(&self, data: &SocketData) -> Result<String> {
        self.render_with_marker("socket", data)
    }

    pub fn render_mount(&self, data: &MountData) -> Result<String> {
        self.render_with_marker("mount", data)
    }

    pub fn render_target(&self, data: &TargetData) -> Result<String> {
        self.render_with_marker("target", data)
    }
}

impl Default for Templates {
    fn default() -> Self {
        Self::new().expect("Failed to initialize templates")
    }
}

/// Clean up unit file content by removing excessive blank lines
fn clean_unit_content(content: &str) -> String {
    let mut result = Vec::new();
    let mut prev_blank = false;

    for line in content.lines() {
        let is_blank = line.trim().is_empty();

        // Skip consecutive blank lines
        if is_blank && prev_blank {
            continue;
        }

        result.push(line);
        prev_blank = is_blank;
    }

    // Remove trailing blank lines
    while result.last().is_some_and(|s| s.trim().is_empty()) {
        result.pop();
    }

    result.join("\n") + "\n"
}

/// Data for service unit template
#[derive(Debug, Serialize)]
pub struct ServiceData {
    pub description: String,
    pub after: Option<String>,
    pub wants: Option<String>,
    pub requires: Option<String>,
    pub service_type: String,
    pub exec: String,
    pub workdir: Option<String>,
    pub user: Option<String>,
    pub group: Option<String>,
    pub restart: String,
    pub restart_sec: u32,
    pub env: Vec<String>,
    pub env_file: Option<String>,
    pub hardening: bool,
    pub wanted_by: String,
}

impl Default for ServiceData {
    fn default() -> Self {
        Self {
            description: String::new(),
            after: Some("network.target".to_string()),
            wants: None,
            requires: None,
            service_type: "simple".to_string(),
            exec: String::new(),
            workdir: None,
            user: None,
            group: None,
            restart: "on-failure".to_string(),
            restart_sec: 5,
            env: Vec::new(),
            env_file: None,
            hardening: false,
            wanted_by: "default.target".to_string(),
        }
    }
}

/// Data for timer unit template
#[derive(Debug, Serialize)]
pub struct TimerData {
    pub description: String,
    pub on_calendar: Option<String>,
    pub on_boot: Option<String>,
    pub on_startup: Option<String>,
    pub on_active: Option<String>,
    pub on_unit_active: Option<String>,
    pub on_unit_inactive: Option<String>,
    pub persistent: bool,
    pub randomize_delay: Option<String>,
    pub unit: String,
    pub wanted_by: String,
}

impl Default for TimerData {
    fn default() -> Self {
        Self {
            description: String::new(),
            on_calendar: None,
            on_boot: None,
            on_startup: None,
            on_active: None,
            on_unit_active: None,
            on_unit_inactive: None,
            persistent: false,
            randomize_delay: None,
            unit: String::new(),
            wanted_by: "timers.target".to_string(),
        }
    }
}

/// Data for path unit template
#[derive(Debug, Serialize)]
pub struct PathData {
    pub description: String,
    pub path_exists: Option<String>,
    pub path_exists_glob: Option<String>,
    pub path_changed: Option<String>,
    pub path_modified: Option<String>,
    pub directory_not_empty: Option<String>,
    pub make_directory: bool,
    pub unit: String,
    pub wanted_by: String,
}

impl Default for PathData {
    fn default() -> Self {
        Self {
            description: String::new(),
            path_exists: None,
            path_exists_glob: None,
            path_changed: None,
            path_modified: None,
            directory_not_empty: None,
            make_directory: false,
            unit: String::new(),
            wanted_by: "default.target".to_string(),
        }
    }
}

/// Data for socket unit template
#[derive(Debug, Serialize, Default)]
pub struct SocketData {
    pub description: String,
    pub listen_stream: Option<String>,
    pub listen_datagram: Option<String>,
    pub listen_fifo: Option<String>,
    pub accept: bool,
    pub max_connections: Option<u32>,
    pub unit: Option<String>,
}

/// Data for mount unit template
#[derive(Debug, Serialize)]
pub struct MountData {
    pub description: String,
    pub what: String,
    pub r#where: String,
    pub fs_type: Option<String>,
    pub options: Option<String>,
    pub wanted_by: String,
}

impl Default for MountData {
    fn default() -> Self {
        Self {
            description: String::new(),
            what: String::new(),
            r#where: String::new(),
            fs_type: None,
            options: None,
            wanted_by: "multi-user.target".to_string(),
        }
    }
}

/// Data for target unit template
#[derive(Debug, Serialize)]
pub struct TargetData {
    pub description: String,
    pub wants: Option<String>,
    pub requires: Option<String>,
    pub after: Option<String>,
    pub wanted_by: String,
}

impl Default for TargetData {
    fn default() -> Self {
        Self {
            description: String::new(),
            wants: None,
            requires: None,
            after: None,
            wanted_by: "default.target".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_template() {
        let templates = Templates::new().unwrap();
        let data = ServiceData {
            description: "Test service".to_string(),
            exec: "/usr/bin/test".to_string(),
            ..Default::default()
        };

        let result = templates.render_service(&data).unwrap();
        assert!(result.contains(MKUNIT_MARKER));
        assert!(result.contains("Description=Test service"));
        assert!(result.contains("ExecStart=/usr/bin/test"));
    }

    #[test]
    fn test_timer_template() {
        let templates = Templates::new().unwrap();
        let data = TimerData {
            description: "Test timer".to_string(),
            on_calendar: Some("daily".to_string()),
            unit: "test.service".to_string(),
            ..Default::default()
        };

        let result = templates.render_timer(&data).unwrap();
        assert!(result.contains("OnCalendar=daily"));
        assert!(result.contains("Unit=test.service"));
    }

    #[test]
    fn test_clean_unit_content() {
        let input = "line1\n\n\nline2\n\n";
        let expected = "line1\n\nline2\n";
        assert_eq!(clean_unit_content(input), expected);
    }
}
