/* Generated with cbindgen:0.26.0 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Returns all the properties of the config.properties content in JSON format
 */
char *get_all_properties(const char *c_config_content);

/**
 * Returns an updated version of the config file with the properties set.
 */
char *set_properties(const char *c_config_content, const char *c_changed_properties);

/**
 * Returns all the properties from the second config file that were not present or
 * differently defined in the template config file.
 */
char *get_properties_diff(const char *c_template_content, const char *c_config_content);
