use crate::communication::KdcComm;
use crate::core::forge::new_nt_srv_inst;
use crate::core::stringifier::ticket_cred_to_string;
use crate::core::CredFormat;
use crate::core::KrbUser;
use crate::core::Vault;
use crate::core::{
    get_impersonation_ticket, get_user_tgt, request_regular_tgs,
    request_s4u2self_tgs, request_tgs, S4u,
};
use crate::error::Result;
use kerberos_crypto::Key;
use log::{debug, info};

/// Main function to request a new TGS for a user for the selected service
pub fn ask_tgs(
    user: KrbUser,
    service: String,
    user_key: Option<&Key>,
    cred_format: CredFormat,
    vault: &mut dyn Vault,
    mut kdccomm: KdcComm,
) -> Result<()> {
    let channel = kdccomm.create_channel(&user.realm)?;

    let tgt = get_user_tgt(user.clone(), user_key, None, vault, &*channel)?;
    debug!("TGT for {} info\n{}", user, ticket_cred_to_string(&tgt, 0));

    let tgs = request_regular_tgs(
        user.clone(),
        new_nt_srv_inst(&service),
        tgt,
        None,
        &mut kdccomm,
    )?;

    info!("Save {} TGS for {} in {}", user, service, vault.id());
    vault.add(tgs)?;

    vault.change_format(cred_format)?;
    return Ok(());
}

/// Main function to perform an S4U2Self operation
pub fn ask_s4u2self(
    user: KrbUser,
    impersonate_user: KrbUser,
    vault: &mut dyn Vault,
    user_key: Option<&Key>,
    cred_format: CredFormat,
    mut kdccomm: KdcComm,
) -> Result<()> {
    let channel = kdccomm.create_channel(&user.realm)?;

    let tgt = get_user_tgt(user.clone(), user_key, None, vault, &*channel)?;
    debug!("TGT for {} info\n{}", user, ticket_cred_to_string(&tgt, 0));

    request_s4u2self_tgs(user, impersonate_user, tgt, vault, &mut kdccomm)?;

    vault.change_format(cred_format)?;

    return Ok(());
}

/// Main function to perform an S4U2Proxy operation
pub fn ask_s4u2proxy(
    user: KrbUser,
    impersonate_user: KrbUser,
    service: String,
    vault: &mut dyn Vault,
    user_key: Option<&Key>,
    cred_format: CredFormat,
    mut kdccomm: KdcComm,
) -> Result<()> {
    let channel = kdccomm.create_channel(&user.realm)?;

    let tgt = get_user_tgt(user.clone(), user_key, None, vault, &*channel)?;
    debug!("TGT for {} info\n{}", user, ticket_cred_to_string(&tgt, 0));

    let s4u2self_tgs = get_impersonation_ticket(
        user.clone(),
        impersonate_user.clone(),
        tgt.clone(),
        vault,
        &mut kdccomm,
    )?;

    info!("Request {} S4U2Proxy TGS for {}", service, impersonate_user);
    let mut tgs_proxy = request_tgs(
        user.clone(),
        user.realm.clone(),
        tgt.clone(),
        S4u::S4u2proxy(s4u2self_tgs.ticket.clone(), service.clone()),
        None,
        &*channel,
    )?;

    if tgs_proxy.is_tgt() {
        let dst_realm = tgs_proxy
            .service_host()
            .ok_or("Unable to get the inter-realm TGT domain")?
            .clone();

        let inter_tgt = request_regular_tgs(
            user.clone(),
            new_nt_srv_inst(&format!("krbtgt/{}", dst_realm)),
            tgt.clone(),
            None,
            &mut kdccomm,
        )?;

        let channel = kdccomm.create_channel(&dst_realm)?;

        tgs_proxy = request_tgs(
            user.clone(),
            dst_realm,
            inter_tgt.clone(),
            S4u::S4u2proxy(tgs_proxy.ticket, service.clone()),
            None,
            &*channel,
        )?;
    }

    debug!(
        "{} S4U2Proxy TGS for {}\n{}",
        service,
        impersonate_user,
        ticket_cred_to_string(&tgs_proxy, 0)
    );

    info!(
        "Save {} S4U2Proxy TGS for {} in {}",
        service,
        impersonate_user,
        vault.id()
    );
    vault.add(tgs_proxy)?;
    vault.change_format(cred_format)?;

    return Ok(());
}
