/* Basic types */
pub const START: u32 = 0x00000;
pub const NONE: u32 = 0x00001;
pub const BOOL: u32 = 0x00002;
pub const ID: u32 = 0x00003;
pub const INT: u32 = 0x00004;
pub const LONG: u32 = 0x00005;
pub const FLOAT: u32 = 0x00006;
pub const DOUBLE: u32 = 0x00007;
pub const STRING: u32 = 0x00008;
pub const BYTES: u32 = 0x00009;
pub const RECTANGLE: u32 = 0x0000A;
pub const FRACTION: u32 = 0x0000B;
pub const BITMAP: u32 = 0x0000C;
pub const ARRAY: u32 = 0x0000D;
pub const STRUCT: u32 = 0x0000E;
pub const OBJECT: u32 = 0x0000F;
pub const SEQUENCE: u32 = 0x00010;
pub const POINTER: u32 = 0x00011;
pub const FD: u32 = 0x00012;
pub const CHOICE: u32 = 0x00013;
pub const POD: u32 = 0x00014;

/* pointers */
pub const POINTER_START: u32 = 0x10000;
pub const POINTER_BUFFER: u32 = 0x10001;
pub const POINTER_META: u32 = 0x10002;
pub const POINTER_DICT: u32 = 0x10003;

/* events */
pub const EVENT_START: u32 = 0x20000;
pub const EVENT_DEVICE: u32 = 0x20001;
pub const EVENT_NODE: u32 = 0x20002;

/* commands */
pub const COMMAND_START: u32 = 0x30000;
pub const COMMAND_DEVICE: u32 = 0x30001;
pub const COMMAND_NODE: u32 = 0x30002;

/* objects */
pub const OBJECT_START: u32 = 0x40000;
pub const OBJECT_PROP_INFO: u32 = 0x40001;
pub const OBJECT_PROPS: u32 = 0x40002;
pub const OBJECT_FORMAT: u32 = 0x40003;
pub const OBJECT_PARAM_BUFFERS: u32 = 0x40004;
pub const OBJECT_PARAM_META: u32 = 0x40005;
pub const OBJECT_PARAM_I_O: u32 = 0x40006;
pub const OBJECT_PARAM_PROFILE: u32 = 0x40007;
pub const OBJECT_PARAM_PORT_CONFIG: u32 = 0x40008;
pub const OBJECT_PARAM_ROUTE: u32 = 0x40009;
pub const OBJECT_PROFILER: u32 = 0x4000A;
pub const OBJECT_PARAM_LATENCY: u32 = 0x4000B;
pub const OBJECT_PARAM_PROCESS_LATENCY: u32 = 0x4000C;
pub const OBJECT_PARAM_TAG: u32 = 0x4000D;
pub const OBJECT_PEER_PARAM: u32 = 0x4000E;
pub const OBJECT_PARAM_DICT: u32 = 0x4000F;

/* vendor extensions */
pub const VENDOR_PIPE_WIRE: u32 = 0x02000000;

pub const VENDOR_OTHER: u32 = 0x7F000000;

// const INFO_BASE: &str = "Spa:";

// #define SPA_TYPE_INFO_Flags			SPA_TYPE_INFO_BASE "Flags"
// #define SPA_TYPE_INFO_FLAGS_BASE		SPA_TYPE_INFO_Flags ":"

// #define SPA_TYPE_INFO_Enum			SPA_TYPE_INFO_BASE "Enum"
// #define SPA_TYPE_INFO_ENUM_BASE			SPA_TYPE_INFO_Enum ":"

// #define SPA_TYPE_INFO_Pod			SPA_TYPE_INFO_BASE "Pod"
// #define SPA_TYPE_INFO_POD_BASE			SPA_TYPE_INFO_Pod ":"

// #define SPA_TYPE_INFO_Struct			SPA_TYPE_INFO_POD_BASE "Struct"
// #define SPA_TYPE_INFO_STRUCT_BASE		SPA_TYPE_INFO_Struct ":"

// #define SPA_TYPE_INFO_Object			SPA_TYPE_INFO_POD_BASE "Object"
// #define SPA_TYPE_INFO_OBJECT_BASE		SPA_TYPE_INFO_Object ":"

// #define SPA_TYPE_INFO_Pointer			SPA_TYPE_INFO_BASE "Pointer"
// #define SPA_TYPE_INFO_POINTER_BASE		SPA_TYPE_INFO_Pointer ":"

// #define SPA_TYPE_INFO_Interface			SPA_TYPE_INFO_POINTER_BASE "Interface"
// #define SPA_TYPE_INFO_INTERFACE_BASE		SPA_TYPE_INFO_Interface ":"

// #define SPA_TYPE_INFO_Event			SPA_TYPE_INFO_OBJECT_BASE "Event"
// #define SPA_TYPE_INFO_EVENT_BASE		SPA_TYPE_INFO_Event ":"

// #define SPA_TYPE_INFO_Command			SPA_TYPE_INFO_OBJECT_BASE "Command"
// #define SPA_TYPE_INFO_COMMAND_BASE		SPA_TYPE_INFO_Command ":"

// struct spa_type_info {
// 	uint32_t type;
// 	uint32_t parent;
// 	const char *name;
// 	const struct spa_type_info *values;
// };

// SPA_API_TYPE bool spa_type_is_a(const char *type, const char *parent)
// {
// 	return type != NULL && parent != NULL && strncmp(type, parent, strlen(parent)) == 0;
// }

// SPA_API_TYPE const char *spa_type_short_name(const char *name)
// {
// 	const char *h;
// 	if ((h = strrchr(name, ':')) != NULL)
// 		name = h + 1;
// 	return name;
// }

// SPA_API_TYPE uint32_t spa_type_from_short_name(const char *name,
// 		const struct spa_type_info *info, uint32_t unknown)
// {
// 	int i;
// 	for (i = 0; info[i].name; i++) {
// 		if (spa_streq(name, spa_type_short_name(info[i].name)))
// 			return info[i].type;
// 	}
// 	return unknown;
// }
// SPA_API_TYPE const char * spa_type_to_name(uint32_t type,
// 		const struct spa_type_info *info, const char *unknown)
// {
// 	int i;
// 	for (i = 0; info[i].name; i++) {
// 		if (info[i].type == type)
// 			return info[i].name;
// 	}
// 	return unknown;
// }

// SPA_API_TYPE const char * spa_type_to_short_name(uint32_t type,
// 		const struct spa_type_info *info, const char *unknown)
// {
// 	const char *n = spa_type_to_name(type, info, unknown);
// 	return n ? spa_type_short_name(n) : NULL;
// }
