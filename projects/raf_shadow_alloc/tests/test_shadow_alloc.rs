use raf_shadow_alloc::{get_available_shadow_stack_size, get_shadow_stack_size, shadow_alloc, shadow_alloc_zeroed};

#[test]
fn test_shadow_alloc() {
    const SIZE: usize = 100;

    shadow_alloc(SIZE, |buf| {
        for idx in 0..SIZE {
            buf[idx] = 5;
        }
    });
}


#[test]
fn test_overflow() {
    let shadow_stack_size = get_shadow_stack_size();
    let page_size = 1 << 16;
    let limit = 2* ((shadow_stack_size / page_size) + 10);
    for _ in 0..limit {
        shadow_alloc(page_size, |buf| {
            buf.fill(0);
        });
    }
}


#[test]
fn test_zeroing() {
    shadow_alloc(100, |buf| {
        buf[0] = 1;
        buf[1] = 2;
        buf[2] = 3;
        buf[3] = 4;
    });

    shadow_alloc(4, |buf| {
        assert_eq!(buf, [1, 2, 3, 4]);
    });

    shadow_alloc_zeroed(4, |buf| {
        assert_eq!(buf, [0, 0, 0, 0]);
    });
}

#[test]
fn test_stack_size() {
    let size = get_available_shadow_stack_size();

    shadow_alloc(100, |_| {
        assert_eq!(get_available_shadow_stack_size(), size - 100);
    });

    assert_eq!(get_available_shadow_stack_size(), size);
}
