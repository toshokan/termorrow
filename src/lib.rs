use flag_mast::*;

mod sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[derive(Default)]
#[repr(C)]
pub struct Termios {
    input_flags: InputMode,
    output_flags: OutputMode,
    control_flags: ControlMode,
    local_flags: LocalMode,
    #[cfg(target_os = "linux")]
    cc_line: std::os::raw::c_uchar,
    #[cfg(target_os = "linux")]
    cc: [std::os::raw::c_uchar; 32],
    #[cfg(target_is = "darwin")]
    cc: [std::os::raw::c_uchar; 20],
    input_speed: Speed,
    output_speed: Speed
}

#[repr(u32)]
enum Speed {
    /// Hang up
    B0 = sys::B0,
    /// 50 baud 
    B50 = sys::B50,
    /// 75 baud 
    B75 = sys::B75,
    /// 110 baud 
    B110 = sys::B110,
    /// 134.5 baud 
    B134 = sys::B134,
    /// 150 baud 
    B150 = sys::B150,
    /// 200 baud 
    B200 = sys::B200,
    /// 300 baud 
    B300 = sys::B300,
    /// 600 baud 
    B600 = sys::B600,
    /// 1200 baud 
    B1200 = sys::B1200,
    /// 1800 baud 
    B1800 = sys::B1800,
    /// 2400 baud 
    B2400 = sys::B2400,
    /// 4800 baud 
    B4800 = sys::B4800,
    /// 9600 baud 
    B9600 = sys::B9600,
    /// 19200 baud 
    B19200 = sys::B19200,
    /// 38400 baud 
    B38400 = sys::B38400,
}

impl Default for Speed {
    fn default() -> Self {
	Speed::B0
    }
}

#[derive(Flags, Default)]
#[flag(name = "BRKINT", method_name = "brkint", value = "sys::BRKINT", doc = "Signal interrupt on break.")]
#[flag(name = "ICRNL", method_name = "icrnl", value = "sys::ICRNL", doc = "Map CR to NL on input.")]
#[flag(name = "IGNBRK", method_name = "ignbrk", value = "sys::IGNBRK", doc = "Ignore break condition.")]
#[flag(name = "IGNCR", method_name = "igncr", value = "sys::IGNCR", doc = "Ignore CR")]
#[flag(name = "IGNPAR", method_name = "ignpar", value = "sys::IGNPAR", doc = "Ignore characters with parity errors.")]
#[flag(name = "INLCR", method_name = "inlcr", value = "sys::INLCR", doc = "Map NL to CR on input.")]
#[flag(name = "INPCK", method_name = "inpck", value = "sys::INPCK", doc = "Enable input parity check.")]
#[flag(name = "ISTRIP", method_name = "istrip", value = "sys::ISTRIP", doc = "Strip character")]
#[flag(name = "IUCLC", method_name = "iuclc", value = "sys::IUCLC", doc = "Map upper-case to lower-case on input (LEGACY).")]
#[flag(name = "IXANY", method_name = "ixany", value = "sys::IXANY", doc = "Enable any character to restart output.")]
#[flag(name = "IXOFF", method_name = "ixoff", value = "sys::IXOFF", doc = "Enable start/stop input control.")]
#[flag(name = "IXON", method_name = "ixon", value = "sys::IXON", doc = "Enable start/stop output control.")]
#[flag(name = "PARMRK", method_name = "parmrk", value = "sys::PARMRK", doc = "Mark parity errors.")]
pub struct InputMode(#[flag_backing_field] u32);

#[derive(Flags, Default)]
#[flag(name = "OPOST", method_name = "opost", value = "sys::OPOST", doc = "Post-process output")]
#[flag(name = "OLCUC", method_name = "olcuc", value = "sys::OLCUC", doc = "Map lower-case to upper-case on output (LEGACY).")]
#[flag(name = "ONLCR", method_name = "onlcr", value = "sys::ONLCR", doc = "Map NL to CR-NL on output.")]
#[flag(name = "OCRNL", method_name = "ocrnl", value = "sys::OCRNL", doc = "Map CR to NL on output.")]
#[flag(name = "ONOCR", method_name = "onocr", value = "sys::ONOCR", doc = "No CR output at column 0.")]
#[flag(name = "ONLRET", method_name = "onlret", value = "sys::ONLRET", doc = "NL performs CR function.")]
#[flag(name = "OFILL", method_name = "ofill", value = "sys::OFILL", doc = "Use fill characters for delay.")]
pub struct OutputMode(#[flag_backing_field] u32);

#[derive(Flags, Default)]
#[flag(name = "CSTOPB", method_name = "cstopb", value = "sys::CSTOPB", doc = "Send two stop bits, else one.")]
#[flag(name = "CREAD", method_name = "cread", value = "sys::CREAD", doc = "Enable receiver.")]
#[flag(name = "PARENB", method_name = "parenb", value = "sys::PARENB", doc = "Parity enable.")]
#[flag(name = "PARODD", method_name = "parodd", value = "sys::PARODD", doc = "Odd parity, else even.")]
#[flag(name = "HUPCL", method_name = "hupcl", value = "sys::HUPCL", doc = "Hang up on last close.")]
#[flag(name = "CLOCAL", method_name = "clocal", value = "sys::CLOCAL", doc = "Ignore modem status lines.")]
pub struct ControlMode(#[flag_backing_field] u32);

#[derive(Flags, Default)]
#[flag(name = "ECHO", method_name = "echo", value = "sys::ECHO", doc = "Enable echo.")]
#[flag(name = "ECHOE", method_name = "echoe", value = "sys::ECHOE", doc = "Echo erase character as error-correcting backspace.")]
#[flag(name = "ECHOK", method_name = "echok", value = "sys::ECHOK", doc = "Echo KILL.")]
#[flag(name = "ECHONL", method_name = "echonl", value = "sys::ECHONL", doc = "Echo NL.")]
#[flag(name = "ICANON", method_name = "icanon", value = "sys::ICANON", doc = "Canonical input (erase and kill processing).")]
#[flag(name = "IEXTEN", method_name = "iexten", value = "sys::IEXTEN", doc = "Enable extended input character processing.")]
#[flag(name = "ISIG", method_name = "isig", value = "sys::ISIG", doc = "Enable signals.")]
#[flag(name = "NOFLSH", method_name = "noflsh", value = "sys::NOFLSH", doc = "Disable flush after interrupt or quit.")]
#[flag(name = "TOSTOP", method_name = "tostop", value = "sys::TOSTOP", doc = "Send SIGTTOU for background output.")]
#[flag(name = "XCASE", method_name = "xcase", value = "sys::XCASE", doc = "Canonical upper/lower presentation (LEGACY).")]
pub struct LocalMode(#[flag_backing_field] u32);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! ptr_offset {
	($t: ty, $f: ident) => {
	    unsafe { (&(*(std::ptr::null::<$t>())).$f as *const _ as usize) }
	}
    }
    
    #[test]
    fn sizes_match() {
	assert_eq!(std::mem::size_of::<crate::sys::termios>(), std::mem::size_of::<Termios>())
    }

    #[test]
    fn fields_match() {
	use sys::termios;
	assert_eq!(ptr_offset!(termios, c_iflag), ptr_offset!(Termios, input_flags));
	assert_eq!(ptr_offset!(termios, c_oflag), ptr_offset!(Termios, output_flags));
	assert_eq!(ptr_offset!(termios, c_cflag), ptr_offset!(Termios, control_flags));
 	assert_eq!(ptr_offset!(termios, c_lflag), ptr_offset!(Termios, local_flags));
	assert_eq!(ptr_offset!(termios, c_line), ptr_offset!(Termios, cc_line));
	assert_eq!(ptr_offset!(termios, c_cc), ptr_offset!(Termios, cc));
	assert_eq!(ptr_offset!(termios, c_ispeed), ptr_offset!(Termios, input_speed));
	assert_eq!(ptr_offset!(termios, c_ospeed), ptr_offset!(Termios, output_speed));
    }
}
