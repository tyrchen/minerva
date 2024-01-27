
$version: "2.0"

namespace com.minerva

structure DatasetInfo {
    @required
    name: String,
    @required
    tableName: String,
    @required
    lastModified: Timestamp,
    @required
    size: Long,
    @required
    fields: DatasetFieldList,
}

list DatasetList {
    member: DatasetInfo,
}

structure DatasetField {
    @required
    name: String,
    @required
    type: String,
    @required
    nullable: Boolean,
}

list DatasetFieldList {
    member: DatasetField,
}
