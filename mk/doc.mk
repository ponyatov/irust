.PHONY: doc
doc:
	rsync -r $(HOME)/metadoc/$(APP)/ doc/$(APP)/
	rsync -r $(HOME)/metadoc/copro/ doc/copro/
	git add $@

.PHONY: doxy
doxy: .doxygen doc/DoxygenLayout.xml doc/logo.png doc
	rm -rf doc/html ; doxygen $< 1>/dev/null
