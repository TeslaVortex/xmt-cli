use xmt_cli::dtqpe_poc;

#[test]
fn test_dtqpe_poc() {
    dtqpe_poc::dtqpe_poc();
    // Add assertions here - currently just calling the function
}

#[test]
fn test_xmoney_integrate_mint() {
    use xmt_cli::commands::xmoney_integrate;
    xmoney_integrate::xmoney_integrate("mint");
    // Assertions can be added once actual output is captured
}

#[test]
fn test_xmoney_integrate_burn() {
    use xmt_cli::commands::xmoney_integrate;
    xmoney_integrate::xmoney_integrate("burn");
    // Assertions can be added for output capture
}
