//! Deployment module

pub mod github;
pub mod netlify;
pub mod vercel;
pub mod s3;

pub use github::GitHubDeployer;
pub use netlify::NetlifyDeployer;
pub use vercel::VercelDeployer;
pub use s3::S3Deployer;