use flag_mast::*;

mod sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

struct Termios {
}


#[derive(Flags)]
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

#[derive(Flags)]
#[flag(name = "OPOST", method_name = "opost", value = "sys::OPOST", doc = "Post-process output")]
#[flag(name = "OLCUC", method_name = "olcuc", value = "sys::OLCUC", doc = "Map lower-case to upper-case on output (LEGACY).")]
#[flag(name = "ONLCR", method_name = "onlcr", value = "sys::ONLCR", doc = "Map NL to CR-NL on output.")]
#[flag(name = "OCRNL", method_name = "ocrnl", value = "sys::OCRNL", doc = "Map CR to NL on output.")]
#[flag(name = "ONOCR", method_name = "onocr", value = "sys::ONOCR", doc = "No CR output at column 0.")]
#[flag(name = "ONLRET", method_name = "onlret", value = "sys::ONLRET", doc = "NL performs CR function.")]
#[flag(name = "OFILL", method_name = "ofill", value = "sys::OFILL", doc = "Use fill characters for delay.")]
pub struct OutputMode(#[flag_backing_field] u32);

#[derive(Flags)]
#[flag(name = "CSTOPB", method_name = "cstopb", value = "sys::CSTOPB", doc = "Send two stop bits, else one.")]
#[flag(name = "CREAD", method_name = "cread", value = "sys::CREAD", doc = "Enable receiver.")]
#[flag(name = "PARENB", method_name = "parenb", value = "sys::PARENB", doc = "Parity enable.")]
#[flag(name = "PARODD", method_name = "parodd", value = "sys::PARODD", doc = "Odd parity, else even.")]
#[flag(name = "HUPCL", method_name = "hupcl", value = "sys::HUPCL", doc = "Hang up on last close.")]
#[flag(name = "CLOCAL", method_name = "clocal", value = "sys::CLOCAL", doc = "Ignore modem status lines.")]
pub struct ControlMode(#[flag_backing_field] u32);

#[derive(Flags)]
#[flag(name = "ECHO", method_name = "echo", value = "sys::ECHO", doc = "Enable echo.")]
#[flag(name = "ECHOE", method_name = "echoe", value = "sys::ECHOE", doc = "Echo erase character as error-correcting backspace.")]
#[flag(name = "ECHOK", method_name = "echok", value = "sys::ECHOK", doc = "Echo KILL.")]
#[flag(name = "ECHONL", method_name = "echonl", value = "sys::ECHONL", doc = "Echo NL.")]
#[flag(name = "ICANON", method_name = "icanon", value = "sys::ICANON", doc = "Canonical input (erase and kill processing).")]
#[flag(name = "IEXTEN", method_name = "iexten", value = "sys::IEXTEN", doc = "Enable extended input character processing.")]
#[flag(name = "ISIG", method_name = "isig", value = "sys::ISIG", doc = "Enable signals.")]
#[flag(name = "NOFLSH", method_name = "noflsh", value = "sys::NOFLSH", doc = "Disable flush after interrupt or quit. ")]
#[flag(name = "TOSTOP", method_name = "tostop", value = "sys::TOSTOP", doc = "Send SIGTTOU for background output.")]
#[flag(name = "XCASE", method_name = "xcase", value = "sys::XCASE", doc = "Canonical upper/lower presentation (LEGACY).")]
pub struct LocalMode(#[flag_backing_field] u32);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
