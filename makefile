PROJECT_NAME ?= contract-runtime
NAMESPACE = kbcs
DOCKER_NS = kchain

BUILD_DIR ?= target
BINARY_FILE = app

SERVICE_NAME = $(DOCKER_NS)-$(PROJECT_NAME)
DOCKER_REGISTRY = harbor-k8s.kingdeeresearch.com

DOCKER_RUN_RUST_IMAGE = harbor-k8s.kingdeeresearch.com/kchain/rust-buildenv:latest
DOCKER_RUN_TOOLS_IMAGE = harbor-k8s.kingdeeresearch.com/kchain/contract-tools:latest

IMAGE_NAME = $(DOCKER_NS)/kbcs-$(PROJECT_NAME)
EXTRA_VERSION ?= $(shell git rev-parse --short HEAD)
IMAGE_FULL_NAME = $(DOCKER_REGISTRY)/$(IMAGE_NAME):$(EXTRA_VERSION)

USERID = $(shell id -u)
DRUN = docker run -i --rm --user=$(USERID):$(USERID) \
	-v $(abspath .):/usr/src/myapp \
	-w /usr/src/myapp

KUBERNETES_FILE = nfs-template.yaml deployment-template.yaml service-template.yaml

NFS_SERVER_TEST="10.244.4.155"
NFS_SERVER_PROD="10.244.4.165"

NFS_PATH_TEST="/mnt/vdb/nfs-data/trustchain/baas"
NFS_PATH_PROD="/PRODDATA/bcmanager/baas"

define deploy
	for item in $(KUBERNETES_FILE); do \
		cat .ci/.kubernetes/$$item | \
		sed -e 's|__APP_LABEL__|$(SERVICE_NAME)|g' | \
		sed -e 's|__IMAGE_FULL_NAME__|$(IMAGE_FULL_NAME)|g' | \
		sed -e 's|__CONTAINER_NAME__|$(SERVICE_NAME)|g' | \
		sed -e 's|__NAMESPACE__|$(NAMESPACE)|g' | \
		sed -e 's|__DEPLOY_NAME__|$(SERVICE_NAME)|g' | \
		sed -e 's|__SERVICE_NAME__|$(SERVICE_NAME)|g' | \
		sed -e 's|__NFS_SERVER__|$(1)|g' | \
		sed -e 's|__NFS_PATH__|$(2)|g' | \
		kubectl apply --record -f - ; \
	done
endef

define deploy19
	for item in $(KUBERNETES_FILE); do \
		cat .ci/.kubernetes/$$item | \
		sed -e 's|__APP_LABEL__|$(SERVICE_NAME)|g' | \
		sed -e 's|__IMAGE_FULL_NAME__|$(IMAGE_FULL_NAME)|g' | \
		sed -e 's|__CONTAINER_NAME__|$(SERVICE_NAME)|g' | \
		sed -e 's|__NAMESPACE__|$(NAMESPACE)|g' | \
		sed -e 's|__DEPLOY_NAME__|$(SERVICE_NAME)|g' | \
		sed -e 's|__SERVICE_NAME__|$(SERVICE_NAME)|g' | \
		sed -e 's|__NFS_SERVER__|$(1)|g' | \
		sed -e 's|__NFS_PATH__|$(2)|g' | \
		kubectl.19 apply --record -f - ; \
	done
endef

help:
	@echo
	@echo "帮助文档："
	@echo "  - make help              查看可用脚本"
	@echo "  - make dep               安装依赖"
	@echo "  - make build             编译可执行文件"
	@echo "  - make docker            编译Docker镜像"
	@echo "  - make deploy-test       部署测试环境"
	@echo "  - make deploy-prod       部署正式环境"
	@echo "  - make clean             清理.build"
	@echo

clean:
	@rm -rf target

prepare:
	@export cargo vendor; cargo fmt; cargo clippy

buildenv: clean
	@docker build -t $(DOCKER_RUN_RUST_IMAGE) image/ -f image/Dockerfile.env
	@docker build -t $(DOCKER_RUN_TOOLS_IMAGE) . -f image/Dockerfile.tools

build:
	$(DRUN) \
	  		$(DOCKER_RUN_RUST_IMAGE) \
       		cargo build --release

docker: build
	@docker build -t $(IMAGE_FULL_NAME) . -f image/Dockerfile.in
	@docker tag $(IMAGE_FULL_NAME) $(DOCKER_REGISTRY)/$(IMAGE_NAME):latest
	@docker push $(IMAGE_FULL_NAME)

deploy-test:
	$(call deploy, $(NFS_SERVER_TEST), $(NFS_PATH_TEST))

deploy-prod:
	$(call deploy19, $(NFS_SERVER_PROD), $(NFS_PATH_PROD))

.PHONY: dep build deploy-test deploy-prod clean
