top_targets     ?= all clean test lint validate push

# traverse subdirs
.ONESHELL:
ifneq ($(subdirs),)
$(top_targets)::
	for dir in $(subdirs); do \
		$(MAKE) -C $$dir $@ ; \
	done
endif

.PHONY: all build clean test lint
