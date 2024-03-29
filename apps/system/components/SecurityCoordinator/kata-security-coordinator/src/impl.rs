// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Kata OS security coordinator seL4 support

use kata_security_interface::DeleteKeyRequest;
use kata_security_interface::GetManifestRequest;
use kata_security_interface::LoadApplicationRequest;
use kata_security_interface::LoadModelRequest;
use kata_security_interface::ReadKeyRequest;
use kata_security_interface::SecurityCoordinatorInterface;
use kata_security_interface::SecurityRequest;
use kata_security_interface::SecurityRequestCapability;
use kata_security_interface::SecurityRequestError;
use kata_security_interface::SizeBufferRequest;
use kata_security_interface::UninstallRequest;
use kata_security_interface::WriteKeyRequest;
use log::trace;
use postcard;

extern "C" {
    static SECURITY_RECV_SLOT: seL4_CPtr;
}

pub struct SeL4SecurityCoordinator {
    // TODO(sleffler): mailbox ipc state
}
impl SeL4SecurityCoordinator {
    pub fn new() -> Self { SeL4SecurityCoordinator {} }
}
pub type KataSecurityCoordinatorInterface = SeL4SecurityCoordinator;

impl SecurityCoordinatorInterface for SeL4SecurityCoordinator {
    fn request(
        &mut self,
        request_id: SecurityRequest,
        request_buffer: &[u8],
        _reply_buffer: &mut [u8],
    ) -> Result<(), SecurityRequestError> {
        use SecurityRequestError::*;

        fn _serialize_failure(e: postcard::Error) -> SecurityRequestError {
            trace!("serialize failed: {:?}", e);
            SreBundleDataInvalid
        }
        fn deserialize_failure(e: postcard::Error) -> SecurityRequestError {
            trace!("deserialize failed: {:?}", e);
            SreBundleDataInvalid
        }

        // TODO(sleffler): mailbox ipc
        match request_id {
            SecurityRequest::SrEcho => {
                trace!("ECHO {:?}", request_buffer);
                // TODO(sleffler): fill-in
                Err(SreEchoFailed)
            }
            SecurityRequest::SrInstall => {
                let mut request = postcard::from_bytes::<InstallRequest>(&request_buffer[..])
                    .map_err(deserialize_failure)?;
                request.set_container_cap(unsafe { SECURITY_RECV_SLOT });
                trace!("INSTALL pkg_contents {:?}", request.pkg_contents);
                // TODO(sleffler): fill-in
                Err(SreInstallFailed)
            }
            SecurityRequest::SrUninstall => {
                let request = postcard::from_bytes::<UninstallRequest>(&request_buffer[..])
                    .map_err(deserialize_failure)?;
                trace!("UNINSTALL {}", request.bundle_id);
                // TODO(sleffler): fill-in
                Err(SreUninstallFailed)
            }
            SecurityRequest::SrSizeBuffer => {
                let request = postcard::from_bytes::<SizeBufferRequest>(&request_buffer[..])
                    .map_err(deserialize_failure)?;
                trace!("SIZE BUFFER bundle_id {}", request.bundle_id);
                // TODO(sleffler): fill-in
                Err(SreSizeBufferFailed)
            }
            SecurityRequest::SrGetManifest => {
                let request = postcard::from_bytes::<GetManifestRequest>(&request_buffer[..])
                    .map_err(deserialize_failure)?;
                trace!("GET MANIFEST bundle_id {}", request.bundle_id);
                // TODO(sleffler): fill-in
                Err(SreGetManifestFailed)
            }
            SecurityRequest::SrLoadApplication => {
                let mut request =
                    postcard::from_bytes::<LoadApplicationRequest>(&request_buffer[..])
                        .map_err(deserialize_failure)?;
                request.set_container_cap(unsafe { SECURITY_RECV_SLOT });
                trace!(
                    "LOAD APPLICATION bundle_id {} app_binary {:?}",
                    request.bundle_id,
                    request.app_binary
                );
                // TODO(sleffler): fill-in
                Err(SreLoadApplicationFailed)
            }
            SecurityRequest::SrLoadModel => {
                let mut request = postcard::from_bytes::<LoadModelRequest>(&request_buffer[..])
                    .map_err(deserialize_failure)?;
                request.set_container_cap(unsafe { SECURITY_RECV_SLOT });
                trace!(
                    "LOAD MODEL bundle_id {} model_id {} model_binary {:?}",
                    request.bundle_id,
                    request.model_id,
                    request.model_binary
                );
                // TODO(sleffler): fill-in
                Err(SreLoadModelFailed)
            }
            SecurityRequest::SrReadKey => {
                let request = postcard::from_bytes::<ReadKeyRequest>(&request_buffer[..])
                    .map_err(deserialize_failure)?;
                trace!("READ KEY bundle_id {} key {}", request.bundle_id, request.key,);
                // TODO(sleffler): fill-in
                Err(SreReadFailed)
            }
            SecurityRequest::SrWriteKey => {
                let request = postcard::from_bytes::<WriteKeyRequest>(&request_buffer[..])
                    .map_err(deserialize_failure)?;
                trace!(
                    "WRITE KEY bundle_id {} key {} value {:?}",
                    request.bundle_id,
                    request.key,
                    request.value,
                );
                // TODO(sleffler): fill-in
                Err(SreWriteFailed)
            }
            SecurityRequest::SrDeleteKey => {
                let request = postcard::from_bytes::<DeleteKeyRequest>(&request_buffer[..])
                    .map_err(deserialize_failure)?;
                trace!("DELETE KEY bundle_id {} key {}", request.bundle_id, request.key,);
                // TODO(sleffler): fill-in
                Err(SreDeleteFailed)
            }
        }
    }
}
