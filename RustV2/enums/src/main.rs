enum IpAddrType {
	// Associates data with each enum type
	// Essentially bundles the enum into a struct
	v4 (u8, u8, u8, u8),
	v6 (String)
}

struct IpAddr {
	ip_type: IpAddrType,
	ip_addr: String,
}

fn main() {
	// Uses the :: syntax for enums
	let IPv4 = IpAddrType::v4;
	let IPv6 = IpAddrType::v6;

	// How this would be done w/ a struct instead of just enums
	//let home = IpAddr {
	//	ip_type: IpAddrType::v4,
	//	ip_addr: String::from("127.0.0.1"),
	//};

	//let loopback = IpAddr{
	//	ip_type: IpAddrType::v6,
	//	ip_addr: String::from("::1");,
	//};

	// Using only enums:
	let home = IpAddrType::v4(127, 0, 0, 1);
	let loopback = IpAddrType::v6(String::from("::1"));
}

fn route(ip_type: IpAddrType) {

}
