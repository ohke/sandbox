vm = ubuntu
host =

launch:
	multipass launch --name ${vm} --cloud-init ./config/cloud-init.yaml 18.04
	multipass mount . ${vm}:/home/ubuntu/work

start:
	multipass start ${vm}
	multipass mount . ${vm}:/home/ubuntu/work

stop:
	multipass stop ${vm}

shell:
	multipass shell ${vm}

setup:
	./script/setup.sh

check:
	sudo ip netns exec host1 ping -c 3 10.0.1.1
	sudo ip netns exec host2 ping -c 3 10.0.0.1

tcpdump:
	sudo ip netns exec host${host} tcpdump -l -S

nc:
	sudo ip netns exec host${host} nc 10.0.0.1 40000

echoserver:
	sudo ip netns exec host${host} ../target/debug/examples/echoserver 10.0.0.1 40000

echoclient:
	sudo ip netns exec host1 ../target/debug/examples/echoclient 10.0.1.1 40000
