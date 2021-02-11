use clap::{ArgMatches, App, Arg};

pub fn create_app() -> ArgMatches {
    App::new("tcp/udp echo application")
        .version("1.0.0")
        .author("sabaniki")
        .about("this is tcp/udp echo server and client program")
        .arg(
            Arg::new("protocol_type")
                .about("protocol type [udp|tcp]")
                .value_name("protocol type")
                .index(1)
                .required(true)
        )
        .arg(
            Arg::new("start_as")
                .about("start as [server|client]")
                .value_name("start as")
                .index(2)
                .required(true)
        )
        .arg(
            Arg::new("address_and_port")
                .about("Specify address and port")
                .value_name("addr:port")
                .index(3)
                .required(true)
        ).get_matches()
}