use kube::api::{Api, Patch, ListParams, DeleteParams, PatchParams,ObjectMeta};
use kube::Client;
use k8s_openapi::api::core::v1::{Pod, PersistentVolumeClaim};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::{CustomResource, CustomResourceExt, Resource};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kube_runtime::{utils::try_flatten_applied, watcher};
use futures::{StreamExt, TryStreamExt};

#[derive(CustomResource, Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[kube(group = "cassandradatacenters.cassandra.datastax.com", version = "v1beta1", kind = "CassandraDatacenter", namespaced, status = "CassandraDatacenterStatus")]
pub struct CassandraDatacenterSpec {
    replace_nodes: Vec<String>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct CassandraDatacenterStatus {
    node_replacements: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    let client = Client::try_default().await?;
    let pod_name = "cluster1-dc2-default-sts-0";
    let pods: Api<Pod> = Api::namespaced(client.clone(), "default");
    let pod_to_replace = pods.get(pod_name).await?;
    let volume = pod_to_replace.spec.and_then(|spec|spec.volumes.into_iter().find(|volume| volume.name=="server-data"));

    let dcs : Api<CassandraDatacenter> = Api::namespaced(client.clone(), "default");
    let spec = CassandraDatacenterSpec {
        replace_nodes: vec![pod_name.to_string()],
    };
    let dc1 = dcs.patch("dc1", &PatchParams::default(), &Patch::Strategic(CassandraDatacenter::new("dc1", spec))).await?;


    let pvcs: Api<PersistentVolumeClaim> = Api::namespaced(client.clone(), "default");

    pvcs.delete(&volume.unwrap().persistent_volume_claim.unwrap().claim_name, &DeleteParams::default()).await?;


    let lp = ListParams::default().fields("metadata.name=dc1").timeout(20);
    let mut stream = dcs.watch(&lp, "0").await?.boxed();

    while let Some(status) = stream.try_next().await? {
    }

    Ok(())
}


