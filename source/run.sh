#run.sh
socket_path=`pwd`"/stratovirt.sock"
kernel_path=`pwd`"/bzImage"
rootfs_path=`pwd`"/rootfs.ext4"

# Make sure qmp can be created.
rm -f ${socket_path}

# Start StratoVirt guest linux machine.
 ./target/debug/stratovirt  \
    -machine type=standard_vm \
    -smp 1 \
    -m 1024m \
    -kernel ${kernel_path} \
    -append console=ttyS0 reboot=k panic=1 root=/dev/vda rw \
    -drive file=${rootfs_path},id=rootfs,readonly=off,direct=off \
    -device virtio-blk-pci,drive=rootfs,bus=pcie.0,addr=0x1 \
    -drive file=/usr/share/edk2/ovmf/OVMF_CODE.fd,if=pflash,readonly=on,unit=0 \
    -drive file=/usr/share/edk2/ovmf/OVMF_VARS.fd,if=pflash,unit=1 \
    -qmp unix:${socket_path},server,nowait \
    -serial stdio    