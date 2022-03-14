vm := green-threads-explained-in-200-lines-of-rust

launch:
	multipass launch --name ${vm} --cloud-init ./cloud-init.yaml 20.04
	multipass mount . ${vm}:/home/ubuntu/work

start:
	multipass start ${vm}
	multipass mount . ${vm}:/home/ubuntu/work

stop:
	multipass stop ${vm}

shell:
	multipass shell ${vm}
