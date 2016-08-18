use rtps::common_types::*;
use rtps::*;

#[test]
fn test_history_cache() {
    let mut hc = HistoryCache::new();
    let buf1 = RcBuffer::from_vec(vec![1,2,3]);
    let buf2 = RcBuffer::from_vec(vec![3,4,5]);

    let cc1 = CacheChange::new(
        ChangeKind::ALIVE,
        Guid::new(),
        InstanceHandle::new(),
        100,
        buf1
    );
    hc.add_change(&cc1).unwrap();

    let cc2 = CacheChange::new(
        ChangeKind::ALIVE,
        Guid::new(),
        InstanceHandle::new(),
        200,
        buf2
    );
    hc.add_change(&cc2).unwrap();

    assert_eq!(hc.get_seq_num_min().unwrap(),100);
    assert_eq!(hc.get_seq_num_max().unwrap(),200);

    hc.remove_change(&cc1).unwrap();

    assert_eq!(hc.get_seq_num_min().unwrap(),200);
    assert_eq!(hc.get_seq_num_max().unwrap(),200);
}