(function() {var implementors = {
"sui_indexer":[["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"sui_indexer/models/addresses/struct.DBAddressStats.html\" title=\"struct sui_indexer::models::addresses::DBAddressStats\">DBAddressStats</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>: FromSql&lt;BigInt, __DB&gt;,</span>"],["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"sui_indexer/models/network_metrics/struct.DBNetworkMetrics.html\" title=\"struct sui_indexer::models::network_metrics::DBNetworkMetrics\">DBNetworkMetrics</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.f64.html\">f64</a>: FromSql&lt;Double, __DB&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>: FromSql&lt;BigInt, __DB&gt;,</span>"],["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"sui_indexer/models/objects/struct.Object.html\" title=\"struct sui_indexer::models::objects::Object\">Object</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.epoch.html\" title=\"struct sui_indexer::schema::objects::columns::epoch\">epoch</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.checkpoint.html\" title=\"struct sui_indexer::schema::objects::columns::checkpoint\">checkpoint</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.version.html\" title=\"struct sui_indexer::schema::objects::columns::version\">version</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.storage_rebate.html\" title=\"struct sui_indexer::schema::objects::columns::storage_rebate\">storage_rebate</a>&gt;, __DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.object_id.html\" title=\"struct sui_indexer::schema::objects::columns::object_id\">object_id</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.object_digest.html\" title=\"struct sui_indexer::schema::objects::columns::object_digest\">object_digest</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.previous_transaction.html\" title=\"struct sui_indexer::schema::objects::columns::previous_transaction\">previous_transaction</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.object_type.html\" title=\"struct sui_indexer::schema::objects::columns::object_type\">object_type</a>&gt;, __DB&gt;,\n    <a class=\"enum\" href=\"sui_indexer/models/owners/enum.OwnerType.html\" title=\"enum sui_indexer::models::owners::OwnerType\">OwnerType</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.owner_type.html\" title=\"struct sui_indexer::schema::objects::columns::owner_type\">owner_type</a>&gt;, __DB&gt;,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.owner_address.html\" title=\"struct sui_indexer::schema::objects::columns::owner_address\">owner_address</a>&gt;, __DB&gt;,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.initial_shared_version.html\" title=\"struct sui_indexer::schema::objects::columns::initial_shared_version\">initial_shared_version</a>&gt;, __DB&gt;,\n    <a class=\"enum\" href=\"sui_indexer/models/objects/enum.ObjectStatus.html\" title=\"enum sui_indexer::models::objects::ObjectStatus\">ObjectStatus</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.object_status.html\" title=\"struct sui_indexer::schema::objects::columns::object_status\">object_status</a>&gt;, __DB&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.has_public_transfer.html\" title=\"struct sui_indexer::schema::objects::columns::has_public_transfer\">has_public_transfer</a>&gt;, __DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"sui_indexer/models/objects/struct.NamedBcsBytes.html\" title=\"struct sui_indexer::models::objects::NamedBcsBytes\">NamedBcsBytes</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"sui_indexer/schema/objects/columns/struct.bcs.html\" title=\"struct sui_indexer::schema::objects::columns::bcs\">bcs</a>&gt;, __DB&gt;,</span>"],["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"sui_indexer/models/network_metrics/struct.DBMoveCallMetrics.html\" title=\"struct sui_indexer::models::network_metrics::DBMoveCallMetrics\">DBMoveCallMetrics</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>: FromSql&lt;BigInt, __DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>: FromSql&lt;Text, __DB&gt;,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()