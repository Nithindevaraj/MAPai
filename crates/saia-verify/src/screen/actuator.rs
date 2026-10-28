use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub success: bool,
    pub window_title: String,
    pub action_performed: String,
    pub details: String,
}

pub struct SpatialActuator;

impl SpatialActuator {
    /// Executes an atomic, spatially-grounded GUI action batch against the target application.
    pub fn execute_action(
        window_title: &str,
        action: &str,
        text: Option<&str>,
        color: Option<&str>,
        commit_strategy: &str,
        clear_first: bool,
    ) -> Result<ActionResult> {
        #[cfg(target_os = "windows")]
        {
            Self::execute_windows(window_title, action, text, color, commit_strategy, clear_first)
        }

        #[cfg(not(target_os = "windows"))]
        {
            Err(anyhow!("SpatialActuator currently only supported on Windows"))
        }
    }

    #[cfg(target_os = "windows")]
    fn execute_windows(
        window_title: &str,
        action: &str,
        text: Option<&str>,
        color: Option<&str>,
        commit_strategy: &str,
        clear_first: bool,
    ) -> Result<ActionResult> {
        let mapai_dir = Path::new(".mapai");
        if !mapai_dir.exists() {
            fs::create_dir_all(mapai_dir)?;
        }

        let input_text = text.unwrap_or("").replace('"', "\\\"");
        let target_color = color.unwrap_or("").replace('"', "\\\"");

        let cs_source = format!(
            r#"using System;
using System.Runtime.InteropServices;
using System.Threading;
using System.Windows.Automation;
using System.Windows.Forms;

class Program {{
    [DllImport("user32.dll")] static extern bool SetProcessDPIAware();
    [DllImport("user32.dll")] static extern IntPtr OpenDesktop(string d, uint f, bool i, uint a);
    [DllImport("user32.dll")] static extern bool SetThreadDesktop(IntPtr h);
    [DllImport("user32.dll")] static extern bool SetForegroundWindow(IntPtr hWnd);
    [DllImport("user32.dll")] static extern void mouse_event(uint dwFlags, uint dwData, uint dx, uint dy, UIntPtr dwExtraInfo);
    [DllImport("user32.dll")] static extern bool SetCursorPos(int X, int Y);
    [DllImport("user32.dll")] static extern void keybd_event(byte bVk, byte bScan, uint dwFlags, UIntPtr dwExtraInfo);

    const uint MOUSEEVENTF_LEFTDOWN = 0x0002;
    const uint MOUSEEVENTF_LEFTUP   = 0x0004;
    const uint KEYEVENTF_KEYUP      = 0x0002;

    const byte VK_CONTROL = 0x11;
    const byte VK_A       = 0x41;
    const byte VK_DELETE  = 0x2E;

    static void Click(int x, int y) {{
        SetCursorPos(x, y);
        Thread.Sleep(80);
        mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, UIntPtr.Zero);
        Thread.Sleep(50);
        mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, UIntPtr.Zero);
        Thread.Sleep(150);
    }}

    static void Drag(int x1, int y1, int x2, int y2) {{
        SetCursorPos(x1, y1);
        Thread.Sleep(80);
        mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, UIntPtr.Zero);
        Thread.Sleep(80);
        int steps = 20;
        for (int i = 1; i <= steps; i++) {{
            int cx = x1 + (x2 - x1) * i / steps;
            int cy = y1 + (y2 - y1) * i / steps;
            SetCursorPos(cx, cy);
            Thread.Sleep(10);
        }}
        Thread.Sleep(80);
        mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, UIntPtr.Zero);
        Thread.Sleep(200);
    }}

    static void SendCtrlA() {{
        keybd_event(VK_CONTROL, 0, 0, UIntPtr.Zero);
        Thread.Sleep(50);
        keybd_event(VK_A, 0, 0, UIntPtr.Zero);
        Thread.Sleep(50);
        keybd_event(VK_A, 0, KEYEVENTF_KEYUP, UIntPtr.Zero);
        Thread.Sleep(50);
        keybd_event(VK_CONTROL, 0, KEYEVENTF_KEYUP, UIntPtr.Zero);
        Thread.Sleep(150);
    }}

    static void SendDelete() {{
        keybd_event(VK_DELETE, 0, 0, UIntPtr.Zero);
        Thread.Sleep(50);
        keybd_event(VK_DELETE, 0, KEYEVENTF_KEYUP, UIntPtr.Zero);
        Thread.Sleep(150);
    }}

    static void Main() {{
        SetProcessDPIAware();
        IntPtr hDesk = OpenDesktop("Default", 0, false, 0x01FF);
        SetThreadDesktop(hDesk);

        var root = AutomationElement.RootElement;
        AutomationElement target = null;
        var names = new System.Collections.Generic.List<string>();
        foreach (AutomationElement c in root.FindAll(TreeScope.Children, Condition.TrueCondition)) {{
            names.Add(c.Current.Name);
            if (c.Current.Name.IndexOf("{window_title}", StringComparison.OrdinalIgnoreCase) >= 0) {{
                target = c;
                break;
            }}
        }}

        if (target == null) {{
            Console.WriteLine("ERR:WindowNotFound. Root children: " + string.Join(" | ", names.ToArray()));
            return;
        }}

        SetForegroundWindow(new IntPtr(target.Current.NativeWindowHandle));
        Thread.Sleep(250);

        bool clearFirst = {clear_first};
        if (clearFirst) {{
            Click(800, 500);
            SendCtrlA();
            SendDelete();
            Thread.Sleep(250);
        }}

        string colorName = "{target_color}";
        if (!string.IsNullOrEmpty(colorName)) {{
            var colorBtn = target.FindFirst(TreeScope.Descendants, new PropertyCondition(AutomationElement.NameProperty, colorName));
            if (colorBtn != null) {{
                var r = colorBtn.Current.BoundingRectangle;
                Click((int)(r.X + r.Width / 2), (int)(r.Y + r.Height / 2));
                Thread.Sleep(200);
            }}
        }}

        string act = "{action}";
        if (act == "text-entry" || act == "text") {{
            var textBtn = target.FindFirst(TreeScope.Descendants, new PropertyCondition(AutomationElement.NameProperty, "Text"));
            if (textBtn != null) {{
                var r = textBtn.Current.BoundingRectangle;
                Click((int)(r.X + r.Width / 2), (int)(r.Y + r.Height / 2));
                Thread.Sleep(250);
            }}

            // Drag text box in canvas center
            Drag(600, 450, 1100, 570);
            Thread.Sleep(300);

            Click(700, 500);
            Thread.Sleep(200);

            // If color specified, ensure selected in floating toolbar
            if (!string.IsNullOrEmpty(colorName)) {{
                var colorBtn = target.FindFirst(TreeScope.Descendants, new PropertyCondition(AutomationElement.NameProperty, colorName));
                if (colorBtn != null) {{
                    var r = colorBtn.Current.BoundingRectangle;
                    Click((int)(r.X + r.Width / 2), (int)(r.Y + r.Height / 2));
                    Thread.Sleep(150);
                }}
                Click(700, 500);
                Thread.Sleep(150);
            }}

            string payload = "{input_text}";
            if (!string.IsNullOrEmpty(payload)) {{
                SendKeys.SendWait(payload);
                Thread.Sleep(300);
            }}

            string commit = "{commit_strategy}";
            if (commit == "click-outside") {{
                Click(1200, 700);
            }} else if (commit == "enter") {{
                SendKeys.SendWait("{{ENTER}}");
            }}
            Thread.Sleep(300);
            Console.WriteLine("OK:TextCommitted");
        }} else if (act == "clear-canvas") {{
            Click(800, 500);
            SendCtrlA();
            SendDelete();
            Console.WriteLine("OK:CanvasCleared");
        }} else if (act == "click-center") {{
            Click(800, 500);
            Console.WriteLine("OK:CenterClicked");
        }} else {{
            Console.WriteLine("ERR:UnknownAction");
        }}
    }}
}}
"#,
            window_title = window_title,
            action = action,
            commit_strategy = commit_strategy,
            input_text = input_text,
            target_color = target_color,
            clear_first = clear_first,
        );

        let runner_cs = mapai_dir.join("spatial_runner.cs");
        let runner_exe = mapai_dir.join("spatial_runner.exe");
        fs::write(&runner_cs, cs_source)?;

        let csc_path = r"C:\Windows\Microsoft.NET\Framework64\v4.0.30319\csc.exe";
        let compile_output = Command::new(csc_path)
            .args([
                "/nologo",
                "/r:C:\\Windows\\Microsoft.NET\\Framework64\\v4.0.30319\\WPF\\UIAutomationClient.dll",
                "/r:C:\\Windows\\Microsoft.NET\\Framework64\\v4.0.30319\\WPF\\UIAutomationTypes.dll",
                "/r:C:\\Windows\\Microsoft.NET\\Framework64\\v4.0.30319\\WPF\\WindowsBase.dll",
                "/r:System.Windows.Forms.dll",
                &format!("/out:{}", runner_exe.to_str().unwrap()),
                runner_cs.to_str().unwrap(),
            ])
            .output()?;

        if !compile_output.status.success() {
            let err = String::from_utf8_lossy(&compile_output.stdout);
            return Err(anyhow!("Failed to compile spatial actuator: {}", err));
        }

        let exec_output = Command::new(&runner_exe).output()?;
        let stdout = String::from_utf8_lossy(&exec_output.stdout);
        let stderr = String::from_utf8_lossy(&exec_output.stderr);

        // Clean up temporary files
        let _ = fs::remove_file(&runner_cs);
        let _ = fs::remove_file(&runner_exe);

        if stdout.contains("OK:") {
            Ok(ActionResult {
                success: true,
                window_title: window_title.to_string(),
                action_performed: action.to_string(),
                details: stdout.trim().to_string(),
            })
        } else {
            let combined = format!("stdout: '{}', stderr: '{}'", stdout.trim(), stderr.trim());
            Err(anyhow!("Spatial execution failed: {}", combined))
        }
    }
}
