
$version: "2.0"

namespace com.minerva

structure DatasetInfo {
    @required
    name: String,
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
}

list DatasetFieldList {
    member: DatasetField,
}
