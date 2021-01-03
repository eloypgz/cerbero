use crate::core::request_tgt;
use crate::core::stringifier::ticket_cred_to_string;
use crate::core::CredFormat;
use crate::core::KrbUser;
use crate::core::Vault;
use crate::error::Result;
use crate::transporter::KerberosTransporter;
use kerberos_crypto::Key;
use log::{debug, info};

/// Main function to ask a TGT
pub fn ask_tgt(
    user: KrbUser,
    user_key: &Key,
    transporter: &dyn KerberosTransporter,
    cred_format: CredFormat,
    vault: &mut dyn Vault,
) -> Result<()> {
    info!("Request TGT for {}", user);
    let tgt_info = request_tgt(user.clone(), user_key, None, transporter)?;

    debug!("TGT for {} info\n{}", user, ticket_cred_to_string(&tgt_info, 0));

    info!("Save {} TGT in {}", user, vault.id());
    vault.add(tgt_info)?;
    vault.change_format(cred_format)?;

    return Ok(());
}
