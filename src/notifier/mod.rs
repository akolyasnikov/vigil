// Vigil
//
// Microservices Status Page
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

pub mod generic;

#[cfg(feature = "notifier-email")]
pub mod email;

#[cfg(feature = "notifier-twilio")]
pub mod twilio;

#[cfg(feature = "notifier-slack")]
pub mod slack;

#[cfg(feature = "notifier-xmpp")]
pub mod xmpp;
