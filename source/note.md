# 笔记

现状

```
[root@stratovirt ~]# hwclock --systohc --verbose
hwclock from util-linux 2.36.1
System Time: 1628903260.197535
Trying to open: /dev/rtc0
Using the rtc interface to the clock.
Last drift adjustment done at 1628903099 seconds after 1969
Last calibration done at 1628903099 seconds after 1969
Hardware clock is on UTC time
Assuming hardware clock is kept in UTC time.
RTC type: 'rtc_cmos'
Using delay: 0.500000 seconds
1628903260.500000 is close enough to 1628903260.500000 (0.000000 < 0.001000)
Set RTC to 1628903260 (1628903260 + 0; refsystime = 1628903260.000000)
Setting Hardware Clock to 01:07:40 = 1628903260 seconds since 1969
ioctl(RTC_SET_TIME) was successful.
Not adjusting drift factor because the --update-drift option was not used.
New /etc/adjtime data:
0.000000 1628903260 0.000000
1628903260
UTC
```

1 guest从rtc 设备获取时间的实现

stratovirt 的RTC 设备直接从宿主机获取时间

2 guest kernel driver和 stratovirt 模拟的rtc设备的交互

sysbus总线的read，write 来读/写 模拟RTC设备的数据
