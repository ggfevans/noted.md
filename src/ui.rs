use colored::Colorize;

pub fn ascii_art() {
    println!(
        "{}",
        r"

          ███╗   ██╗ ██████╗ ████████╗███████╗██████╗    ███╗   ███╗██████╗
          ████╗  ██║██╔═══██╗╚══██╔══╝██╔════╝██╔══██╗   ████╗ ████║██╔══██╗
          ██╔██╗ ██║██║   ██║   ██║   █████╗  ██║  ██║   ██╔████╔██║██║  ██║
          ██║╚██╗██║██║   ██║   ██║   ██╔══╝  ██║  ██║   ██║╚██╔╝██║██║  ██║
          ██║ ╚████║╚██████╔╝   ██║   ███████╗██████╔╝██╗██║ ╚═╝ ██║██████╔╝
          ╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚══════╝╚═════╝ ╚═╝╚═╝     ╚═╝╚═════╝
        "
        .bright_blue()
    );
    println!(
        "{}",
        "-------------------------------------------------".dimmed()
    );
}
