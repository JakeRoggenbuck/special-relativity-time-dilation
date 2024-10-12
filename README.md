# special-relativity-time-dilation
Calculate the time dilation by velocity as described by special relativity using Rust! :crab:

```rs
fn time_dilation_by_velocity(t: f64, v: f64) -> f64 {
    t / (1.0 - ((v * v) / (C * C))).sqrt()
}
```

![image](https://github.com/user-attachments/assets/337346ae-f397-42f6-9155-cd2db4ffdc89)
