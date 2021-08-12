# 琐碎记录



## 有用的代码
可参考[Qemu的实现](./mc146818rtc.c) 后续还会参考Qemu的[/hw/rtc](https://github.com/qemu/qemu/tree/master/hw/rtc)

[linux中的mc146818寄存器   linux/mc146818rtc.h](https://github.com/spotify/linux/blob/master/include/linux/mc146818rtc.h)

[linux中RTC相关的有关时间的数据结构](https://github.com/spotify/linux/blob/master/include/linux/rtc.h)

[Qemu基准测试](https://github.com/ahorn/benchmarks/tree/master/qemu-hw/rtc)

[测试rtc功能的代码 C的实现](./source/rtc-example.c) 

## 可能有帮助的文章

标题 并非 文章原本发布时的标题，针对自己的理解写了一下

[从clocksource说明了下kvm-clcok ](https://cloud.tencent.com/developer/article/1087415)    [linux下的**kvmclock.c**源代码](https://github.com/torvalds/linux/blob/master/arch/x86/kernel/kvmclock.c)


## 尚未理解

### 0808

怎么向 hwclock 的 ioctl 暴露接口  [linux/rtc.h](https://github.com/spotify/linux/blob/master/include/linux/rtc.h)
```
struct rtc_class_ops {
	int (*open)(struct device *);
	void (*release)(struct device *);
	int (*ioctl)(struct device *, unsigned int, unsigned long);
	int (*read_time)(struct device *, struct rtc_time *);
	int (*set_time)(struct device *, struct rtc_time *);
	int (*read_alarm)(struct device *, struct rtc_wkalrm *);
	int (*set_alarm)(struct device *, struct rtc_wkalrm *);
	int (*proc)(struct device *, struct seq_file *);
	int (*set_mmss)(struct device *, unsigned long secs);
	int (*irq_set_state)(struct device *, int enabled);
	int (*irq_set_freq)(struct device *, int freq);
	int (*read_callback)(struct device *, int data);
	int (*alarm_irq_enable)(struct device *, unsigned int enabled);
	int (*update_irq_enable)(struct device *, unsigned int enabled);
};
```
