#[macro_export]
macro_rules! format_emojis {
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        let new = res.replace(":build:", "<:build:878027836319989790>")
            .replace(":amber:", "<:amber:878027835531468801>")
            .replace(":amethyst:", "<:amethyst:878027835959296010>")
            .replace(":artifact:", "<:artifact:878027835913142292>")
            .replace(":caulk:", "<:caulk:878027835959296011>")
            .replace(":chalk:", "<:chalk:878027836261294091>")
            .replace(":cobalt:", "<:cobalt:878027836072542238>")
            .replace(":diamond:", "<:diamond:878027836093526036>")
            .replace(":garnet:", "<:garnet:878027836093521940>")
            .replace(":gold:", "<:gold:878027835808301108>")
            .replace(":iodine:", "<:iodine:878027836273864774>")
            .replace(":marble:", "<:marble:878027836093521941>")
            .replace(":mercury:", "<:mercury:878027836093521933>")
            .replace(":quartz:", "<:quartz:878027835929931907>")
            .replace(":ruby:", "<:ruby:878027836248690788>")
            .replace(":rust:", "<:rust:878027836210941953>")
            .replace(":shale:", "<:shale:878027835808301109>")
            .replace(":sulfur:", "<:sulfur:878027836278063174>")
            .replace(":tar:", "<:tar:878027836504559626>")
            .replace(":uranium:", "<:uranium:878027836269674537>")
            .replace(":zillion:", "<:zillion:878027836093521942>")
            .replace(":space:", "<:space:880724667235708959> ")
            .replace(":time:", "<:time:880724667692896306>")
            .replace(":mind:", "<:mind:880724667235708960> ")
            .replace(":heart:", "<:heart:880724667197956098>")
            .replace(":hope:", "<:hope:880724667579637790>")
            .replace(":rage:", "<:rage:880725152973869096>")
            .replace(":breath:", "<:breath:880724667629973504>")
            .replace(":blood:", "<:blood:880724667588018226>")
            .replace(":life:", "<:life:880724667588026378>")
            .replace(":doom:", "<:doom:880724667609002014>")
            .replace(":light:", "<:light:880724667269283853>")
            .replace(":void:", "<:void:880724667621601290>");
        new
    }}
}

#[macro_export]
macro_rules! format_items {
    ($($arg:tt)*) => {{
        let mut res = std::fmt::format(format_args!($($arg)*));
        res = res.replace("disk", "Sburb Disk");
        res
    }}
}


#[macro_export]
macro_rules! format_all {
    ($($arg:tt)*) => {{
        let mut res = std::fmt::format(format_args!($($arg)*));
        res = format_emojis!("{}", res);
        res = format_items!("{}", res); 
        res
    }}
}