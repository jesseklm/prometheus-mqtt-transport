.PHONY: all build strip install clean

all: build strip install

build:
	env PATH=${PATH}:${HOME}/.cargo/bin cargo build --release

strip: build
	strip --strip-all target/release/prom2mqtt-export target/release/prom2mqtt-fetch

clean:
	env PATH=${PATH}:${HOME}/.cargo/bin cargo clean

install: strip
	test -d $(DESTDIR)/usr/sbin || mkdir -m 0755 -p $(DESTDIR)/usr/sbin
	test -d $(DESTDIR)/lib/systemd/system/ || mkdir -m 0755 -p $(DESTDIR)/lib/systemd/system/
	install -m 0755 target/release/prom2mqtt-export $(DESTDIR)/usr/sbin
	install -m 0755 target/release/prom2mqtt-fetch $(DESTDIR)/usr/sbin
	install -m 0644 systemd/prom2mqtt-export.service $(DESTDIR)/lib/systemd/system/
	install -m 0644 systemd/prom2mqtt-fetch.service $(DESTDIR)/lib/systemd/system/
	systemctl daemon-reload

uninstall:
	/bin/rm -f $(DESTDIR)/usr/sbin/prom2mqtt-export $(DESTDIR)/lib/systemd/system/prom2mqtt-export.service
	/bin/rm -f $(DESTDIR)/usr/sbin/prom2mqtt-fetch $(DESTDIR)/lib/systemd/system/prom2mqtt-fetch.service

