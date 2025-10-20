pub mod erc20 {
    use ethers::prelude::abigen;
    abigen!(
        ERC20,
        r#"[ 
            event Transfer(address indexed from, address indexed to, uint256 value)
            function transfer(address to, uint256 value) external returns (bool)
        ]"#
    );
}
