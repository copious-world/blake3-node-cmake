//
//
#include "node_blake3_mod.h"

//-------------------------------

#include <map>

using namespace Nan;
using namespace std;

//-------------------------------


static void AtNodeExit(void*) {
	// maybe destroy tables
}


namespace node {
namespace node_blake3 {

	map<uint32_t,blake332 *> g_blake3_states;
	uint32_t g_hash_count = 0;

	NAN_METHOD(init) {
		Nan::HandleScope scope;
		int err;
		uint32_t seed = Nan::To<uint32_t>(info[0]).FromJust();
		g_hash_count++;
		g_blake3_states[g_hash_count] = new blake332(seed);
		//
		info.GetReturnValue().Set(Nan::New<Number>(g_hash_count));
	}

	NAN_METHOD(update) {
		Nan::HandleScope scope;
		uint32_t key = Nan::To<uint32_t>(info[0]).FromJust();
		//
		blake332 *blake3 = g_blake3_states[key];
		if ( blake3 ) {
			//
			v8::Isolate* isolate = info.GetIsolate();
			v8::String::Utf8Value str(isolate, info[1]);
			std::string s(*str);
			//
			bool rslt = blake3->add(s.c_str(),s.size());
			info.GetReturnValue().Set(Nan::New<Boolean>(rslt));
		} else {
			info.GetReturnValue().Set(Nan::New<Boolean>(false));
		}
	}

	NAN_METHOD(get_hash) {
		Nan::HandleScope scope;
		uint32_t key = Nan::To<uint32_t>(info[0]).FromJust();
		//
		blake332 *blake3 = g_blake3_states[key];
		if ( blake3 ) {
			uint32_t hash_val = blake3->hash();
			info.GetReturnValue().Set(Nan::New<Number>(hash_val));
		} else {
			info.GetReturnValue().Set(Nan::New<Boolean>(false));
		}
	}

	NAN_METHOD(reset) {
		Nan::HandleScope scope;
		uint32_t key = Nan::To<uint32_t>(info[0]).FromJust();
		//
		blake332 *blake3 = g_blake3_states[key];
		if ( blake3 ) {
			uint32_t seed = Nan::To<uint32_t>(info[1]).FromJust();
			g_blake3_states[key] = new blake332(seed);
			delete blake3;
			info.GetReturnValue().Set(Nan::New<Boolean>(true));
		} else {
			info.GetReturnValue().Set(Nan::New<Boolean>(false));
		}
	}

	NAN_METHOD(remove) {
		Nan::HandleScope scope;
		uint32_t key = Nan::To<uint32_t>(info[0]).FromJust();
		//
		blake332 *blake3 = g_blake3_states[key];
		if ( blake3 ) {
			delete blake3;
			g_blake3_states[key] = NULL;
			delete g_blake3_states[key];
			info.GetReturnValue().Set(Nan::New<Boolean>(true));
		} else {
			info.GetReturnValue().Set(Nan::New<Boolean>(false));
		}
	}


	NAN_METHOD(hash_once_32) {
		Nan::HandleScope scope;
		//
		v8::Isolate* isolate = info.GetIsolate();
		v8::String::Utf8Value str(isolate, info[0]);
		std::string s(*str);

		uint32_t seed = Nan::To<uint32_t>(info[1]).FromJust();
		//
		uint32_t hash = blake332::hash(s.c_str(),s.size(),seed);
		//
		info.GetReturnValue().Set(Nan::New<Number>(hash));
	}


	// Init module
	static void Init(Local<Object> target) {
		//
		Nan::SetMethod(target, "init", init);
		Nan::SetMethod(target, "update", update);
		Nan::SetMethod(target, "get_hash", get_hash);
		Nan::SetMethod(target, "reset", reset);
		Nan::SetMethod(target, "remove", remove);
		//
		Nan::SetMethod(target, "hash_once_32", hash_once_32);
		//
		Isolate* isolate = target->GetIsolate();
		AddEnvironmentCleanupHook(isolate,AtNodeExit,nullptr);
		//node::AtExit(AtNodeExit);
	}

}
}

//-------------------------------

NODE_MODULE(blake3, node::node_blake3::Init);
