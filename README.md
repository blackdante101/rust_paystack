[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![LinkedIn][linkedin-shield]][linkedin-url]
[![Crate Badge]][Crate] [![Repo Badge]][Repo] [![Docs Badge]][Docs] [![License Badge]][License] 

a rust library for interacting with Paystack API
## Getting Started
run this command in your project directory
```no_run
cargo add rust_paystack
```
Including the library in your project:
```no_run
use rust_paystack::Paystack;
```
## Creating a new instance
when creating a new instance, api key should be parsed to string
```no_run
let rust_p = RustPaystack::new(PAYSTACK_SECRET_KEY.to_string());
```
#Initializing a transaction
```no_run
#[tokio::main]
!async fn main() {
    let rust_p = RustPaystack::new(PAYSTACK_SECRET_KEY.to_string());
    let req = rust_p.initialize_transaction( "test@testmail.com", 10.50).await;
     
    println!("{:?}", req);
}
```
## Verfiying a transaction
```no_run
#[tokio::main]
async fn main() {
    let rust_p = RustPaystack::new(PAYSTACK_SECRET_KEY.to_string());
    let req = rust_p.verify_payment("reference").await;
     
     println!("{:?}", req);
}
 ```
## Contributing
Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<!-- CONTACT -->
## Contact

Abukari Einus - einusabukari@gmail.com

Project Link: [https://github.com/blackdante101/rust_paystack](https://github.com/blackdante101/rust_paystack)

[contributors-shield]: https://img.shields.io/github/contributors/blackdante101/Best-README-Template.svg?style=for-the-badge
[contributors-url]: https://github.com/blackdante101/rust_paystack/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/blackdante101/Best-README-Template.svg?style=for-the-badge
[forks-url]: https://github.com/blackdante101/rust_paystack/network/members
[stars-shield]: https://img.shields.io/github/stars/blackdante101/Best-README-Template.svg?style=for-the-badge
[stars-url]: https://github.com/blackdante101/rust_paystack/stargazers
[issues-shield]: https://img.shields.io/github/issues/blackdante101/Best-README-Template.svg?style=for-the-badge
[issues-url]: https://github.com/blackdante101/rust_paystack/issues
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://www.linkedin.com/in/abukari-einus-913721177/
[Repo]: https://github.com/blackdante101/rust_paystack
[Docs]: https://docs.rs/rust_paystack
[Crate]: https://crates.io/crates/rust_paystack
[Crate Badge]: https://img.shields.io/crates/v/rust_paystack?logo=rust&style=flat-square&color=E05D44
[Repo Badge]: https://img.shields.io/badge/repo-blackdante101/rust_paytack-1370D3?style=flat-square&logo=github
[License Badge]: https://img.shields.io/crates/l/rust_paystack?style=flat-square&color=1370D3
[Docs Badge]: https://img.shields.io/badge/docs-rust_paystack-1370D3?style=flat-square&logo=rust
[`cargo-generate`]: https://crates.io/crates/cargo-generate
[License]: ./LICENSE