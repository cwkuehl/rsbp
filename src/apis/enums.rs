/// Aufrufarten für einen Dialog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DialogTypeEnum {
    /// Ohne
    Without,
    /// Neu
    New,
    /// Kopieren
    Copy,
    /// Kopieren 2
    Copy2,
    /// Ändern
    Edit,
    /// Löschen
    Delete,
    /// Stornieren
    Reverse,
}

/// Permissions for users.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionEnum {
    /// Without any permission
    Without = -1,
    /// Normal user permission
    User = 0,
    /// Administrator permission
    Admin = 1,
    /// All permissions
    All = 2,
}

impl PermissionEnum {
    pub fn to_i32(e: PermissionEnum) -> i32 {
        return match e {
            PermissionEnum::Without => -1,
            PermissionEnum::User => 0,
            PermissionEnum::Admin => 1,
            PermissionEnum::All => 2,
        };
    }
}

/// Direction of serching.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchDirectionEnum {
    /// no direction
    None = -1,
    /// first entry
    First = 0,
    /// previous entry
    Back = 1,
    /// next entry
    Forward = 2,
    /// last entry
    Last = 3,
}
