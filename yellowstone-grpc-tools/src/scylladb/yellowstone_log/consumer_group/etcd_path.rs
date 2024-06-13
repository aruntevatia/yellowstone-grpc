use {
    crate::scylladb::types::{ConsumerGroupId, ConsumerId, ProducerId, ShardId},
    tracing::trace, uuid::Uuid,
};

pub fn get_instance_lock_name_path_v1(
    consumer_group_id: ConsumerGroupId,
    consumer_id: ConsumerId,
) -> String {
    let uuid = Uuid::from_bytes(consumer_group_id);
    let uuid_str = uuid.to_string();
    format!("v1#lock#cg-{uuid_str}#i-{consumer_id}")
}

pub fn get_instance_lock_prefix_v1(consumer_group_id: ConsumerGroupId) -> String {
    let uuid = Uuid::from_bytes(consumer_group_id);
    let uuid_str = uuid.to_string();
    format!("v1#lock#cg-{uuid_str}#i-")
}

pub fn get_instance_fencing_token_key_path_v1(
    consumer_group_id: ConsumerGroupId,
    consumer_id: ConsumerId,
) -> String {
    let uuid = Uuid::from_bytes(consumer_group_id);
    let uuid_str = uuid.to_string();
    format!("v1#fencing-token#cg-{uuid_str}#i-{consumer_id}")
}

pub fn get_producer_lock_path_v1(producer_id: ProducerId) -> String {
    let producer_id_num = u8::from_be_bytes(producer_id);
    format!("v1#lock#producers#p-{:0>4}", producer_id_num)
}

pub fn get_producer_id_from_lock_key_v1(lock_key: &[u8]) -> anyhow::Result<ProducerId> {
    let s = String::from_utf8_lossy(lock_key);
    let number_part = s
        .split("#")
        .skip(3)
        .next()
        .ok_or(anyhow::anyhow!("invalid lock key format"))?;
    trace!("get_producer_id_from_lock_key_v1 -- number_part : {number_part}");
    let producer_id = &number_part[2..].parse::<u8>()?;
    Ok([*producer_id])
}

pub fn get_producer_lock_prefix_v1() -> String {
    String::from("v1#lock#producers#p-")
}

pub fn get_producer_fencing_token_key_path_v1(producer_id: ProducerId) -> String {
    let producer_id_num = u8::from_be_bytes(producer_id);
    format!("v1#fencing-token#producers#p-{:0>4}", producer_id_num)
}

pub fn get_shard_fencing_token_key_path_v1(producer_id: ProducerId, shard_id: ShardId) -> String {
    let producer_id_num = u8::from_be_bytes(producer_id);
    format!(
        "v1#fencing-token#shards#p-{:0>4}#s-{:0>5}",
        producer_id_num, shard_id
    )
}
