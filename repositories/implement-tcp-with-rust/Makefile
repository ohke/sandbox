launch:
	multipass launch --name ubuntu --cloud-init ./config/cloud-init.yaml 18.04
	multipass mount . ubuntu:/home/ubuntu/work

start:
	multipass start ubuntu
	multipass mount . ubuntu:/home/ubuntu/work

stop:
	multipass stop ubuntu

shell:
	multipass shell ubuntu

setup:
	./script/setup.sh
