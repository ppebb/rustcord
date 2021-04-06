#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum ResposeResult {
    /// everything is good
    Ok = 0,
    /// Discord isn't working
    ServiceUnavailable = 1,
    /// the SDK version may be outdated
    InvalidVersion = 2,
    /// an internal error on transactional operations
    LockFailed = 3,
    /// something on our side went wrong
    InternalError = 4,
    /// the data you sent didn't match what we expect
    InvalidPayload = 5,
    /// that's not a thing you can do
    InvalidCommand = 6,
    /// you aren't authorized to do that
    InvalidPermissions = 7,
    /// couldn't fetch what you wanted
    NotFetched = 8,
    /// what you're looking for doesn't exist
    NotFound = 9,
    /// user already has a network connection open on that channel
    Conflict = 10,
    /// activity secrets must be unique and not match party id
    InvalidSecret = 11,
    /// join request for that user does not exist
    InvalidJoinSecret = 12,
    /// you accidentally set an ApplicationId in your UpdateActivity() payload
    NoEligibleActivity = 13,
    /// your game invite is no longer valid
    InvalidInvite = 14,
    /// the internal auth call failed for the user, and you can't do this
    NotAuthenticated = 15,
    /// the user's bearer token is invalid
    InvalidAccessToken = 16,
    /// access token belongs to another application
    ApplicationMismatch = 17,
    /// something internally went wrong fetching image data
    InvalidDataUrl = 18,
    /// not valid Base64 data
    InvalidBase64 = 19,
    /// you're trying to access the list before creating a stable list with Filter()
    NotFiltered = 20,
    /// the lobby is full
    LobbyFull = 21,
    /// the secret you're using to connect is wrong
    InvalidLobbySecret = 22,
    /// file name is too long
    InvalidFilename = 23,
    /// file is too large
    InvalidFileSize = 24,
    /// the user does not have the right entitlement for this game
    InvalidEntitlement = 25,
    /// Discord is not installed
    NotInstalled = 26,
    /// Discord is not running
    NotRunning = 27,
    /// insufficient buffer space when trying to write
    InsufficientBuffer = 28,
    /// user cancelled the purchase flow
    PurchaseCancelled = 29,
    /// Discord guild does not exist
    InvalidGuild = 30,
    /// the event you're trying to subscribe to does not exist
    InvalidEvent = 31,
    /// Discord channel does not exist
    InvalidChannel = 32,
    /// the origin header on the socket does not match what you've registered (you should not see this)
    InvalidOrigin = 33,
    /// you are calling that method too quickly
    RateLimited = 34,
    /// the OAuth2 process failed at some point
    OAuth2Error = 35,
    /// the user took too long selecting a channel for an invite
    SelectChannelTimeout = 36, 
    /// took too long trying to fetch the guild
    GetGuildTimeout = 37,
    /// push to talk is required for this channel
    SelectVoiceForceRequired = 38,
    /// that push to talk shortcut is already registered
    CaptureShortcutAlreadyListening = 39,
    /// your application cannot update this achievement
    UnauthorizedForAchievement = 40,
    /// the gift code is not valid
    InvalidGiftCode = 41,
    /// something went wrong during the purchase flow
    PurchaseError = 42,
    /// purchase flow aborted because the SDK is being torn down
    TransactionAborted = 43,
}

pub enum LogLevel {
    /// NOTE: ORIGINALLY DID NOT HAVE VALUES.
    /// Log only errors
    Error = 0,
    /// Log warnings and errors
    Warning	= 1,
    /// Log info, warnings, and errors
    Info = 2,
    /// Log all the things!
    Debug = 3,
}

pub enum CreateFlags {
    /// NOTE: ORIGINALLY DID NOT HAVE VALUES.
    /// Requires Discord to be running to play the game
    Default = 0, 
    /// Does not require Discord to be running, use this on other platforms
    NoRequireDiscord = 1,
}
