use mdsn::Dsn;
use std::str::FromStr;

fn main() -> Result<(), mdsn::DsnError> {
    // The two styles are equivalent.
    let dsn = Dsn::from_str("taos://root:taosdata@host1:6030,host2:6030/db")?;
    let dsn: Dsn = "taos://root:taosdata@host1:6030,host2:6030/db".parse()?;

    assert_eq!(dsn.driver, "taos");
    assert_eq!(dsn.username.unwrap(), "root");
    assert_eq!(dsn.password.unwrap(), "taosdata");
    assert_eq!(dsn.subject.unwrap(), "db");
    assert_eq!(dsn.addresses.len(), 2);
    assert_eq!(
        dsn.addresses,
        vec![
            mdsn::Address::new("host1", 6030),
            mdsn::Address::new("host2", 6030),
        ]
    );
    Ok(())
}
