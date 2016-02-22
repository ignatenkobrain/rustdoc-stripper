// Copyright 2015 Gomez Guillaume
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod regenerate;
pub mod strip;
pub mod types;
pub mod utils;
mod consts;

pub use regenerate::{
    parse_cmts,
    regenerate_comments,
    regenerate_doc_comments,
};
pub use strip::strip_comments;
pub use consts::{
    FILE,
    FILE_COMMENT,
    MOD_COMMENT,
    END_INFO,
    OUTPUT_COMMENT_FILE,
};
pub use types::{
    EventType,
    Type,
    TypeStruct,
};
pub use utils::{
    write_comment,
    write_item_doc,
    write_file,
    write_file_name,
    write_file_comment,
    loop_over_files,
};