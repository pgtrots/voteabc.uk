NSPL_CSV = NSPL_AUG_2019_UK/Data/NSPL_AUG_2019_UK.csv
ROOT_DIR:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

data/postcodes.csv:
	xsv join pcon data/pcon-ids.csv pcon data/$(NSPL_CSV) | xsv select "pcd,id" > data/postcodes.csv
	sed -i "1s/pcd/postcode/" data/postcodes.csv
	sed -i "1s/id/constituency_id/" data/postcodes.csv

data/voteabc.db:
	gzip -dc data/voteabc.data.sql.zip | sqlite3 data/voteabc.db

.PHONY: build-frontend
build-frontend:
	cd frontend ;\
	npm run export

.PHONY: build-server
build-server:
	cd server ;\
	cargo build --release

.PHONY: build-server-arm
build-server-arm:
	cd server ;\
	cross build --release --target armv7-unknown-linux-gnueabihf

docker/voteabc-server.tar: build-server
	cd server ;\
	docker build -t voteabc-server . &&\
	docker save -o ../docker/voteabc-server.tar voteabc-server

.PHONY: build-package
build-package: build-server-arm build-frontend
	rm -rf __package__
	mkdir __package__
	ln -fs $(ROOT_DIR)/server/target/armv7-unknown-linux-gnueabihf/release/voteabc-server $(ROOT_DIR)/__package__/voteabc-server
	ln -fs $(ROOT_DIR)/frontend/__sapper__/export $(ROOT_DIR)/__package__/static
	ln -fs $(ROOT_DIR)/data/voteabc.db $(ROOT_DIR)/__package__/voteabc.db

.PHONY: deploy-package
deploy-package:
	ansible-playbook ansible/deploy_package.yml -u $$SSH_USER -i '$$SSH_HOST,' -e "package=../__package__"
