//! Goals:
//! - No need for downstream libraries to depend upon libsignal-service-rs.
//! - Make messages sent from others and from oneselves on other devices look identical (except for of course the sender), instead of having a distinct SyncMessage.
//! - Downstream libraries should not need to parse fields of messages to find out e.g. which message was quoted.
//! - There should be a clear differentiation between fields used by consumers of presage, fields required by presage to work (`secrets` and `metadata`).
//! 
//! Conventions:
//! - Types ending in "Id" references something that is likely available locally and does not need to be fetched.
//! - Types ending in "Pointer" references something that likely needs to be fetched from the internet.

// Take those from elsewhere.
type ServiceId = ();
type GroupV2Id = ();
type PhoneNumber = ();
type ProfileKey = ();
type ProfileKeyCredentials = ();
type GroupMasterKey = ();
type DateTime<T> = T;
type Utc = ();
type Duration = ();
type Uuid = ();

// TODO:
type AvatarPointer = ();
type Timer = ();
type Member = ();
type PendingMember = ();
type RequestingMember = ();
type AttachmentPointer = ();
type BodyRange = ();
type Preview = ();
type ContactInformation = ();
type DeviceId = ();
type AccessControl = ();

pub struct ContactId(ServiceId);

pub struct GroupId(GroupV2Id);

pub enum ThreadId {
    Contact(ContactId),
    Group(GroupId),
}

pub struct Contact {
    pub id: ContactId,

    pub phone_number: Option<PhoneNumber>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub is_blocked: bool,

    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub about: Option<String>,
    pub about_emoji: Option<String>,
    pub avatar: Option<AvatarPointer>,
    pub disappearing_messages_timer: Option<Timer>,

    pub secrets: ContactSecrets,
}

pub struct ContactSecrets {
    pub profile_key: ProfileKey,
    pub profile_key_credentials: ProfileKeyCredentials
}

pub struct Group {
    pub id: GroupId,

    pub name: String,
    pub description: Option<String>,
    pub avatar: Option<AvatarPointer>,
    pub announcement_only: bool,
    pub disappearing_messages_timer: Option<Timer>,

    pub members: Vec<Member>,
    pub pending_members: Vec<PendingMember>,
    pub requesting_members: Vec<RequestingMember>,
    pub invite_link_password: Vec<u8>,

    pub access_control: Option<AccessControl>,

    pub secrets: GroupSecrets,
}

pub struct GroupSecrets {
    pub master_key: GroupMasterKey,
    pub revision: i32
}

pub struct MessageId {
    pub thread: ThreadId,
    pub sender: ContactId,
    pub timestamp: DateTime<Utc>,
}

pub struct Message {
    pub id: MessageId,

    pub metadata: MessageMetadata,
    pub content: MessageContent,
}

pub struct MessageMetadata {
    pub needs_receipt: bool,
    pub unidentified_sender: bool,
    pub was_plaintext: bool,
    pub server_guid: Option<Uuid>,
    pub sender_device_id: DeviceId,
}

pub enum MessageContent {
    Text(TextMessage),
    Reaction(ReactionMessage),
    Receipt(ReceiptMessage),
    Typing(TypingMessage),
    Deletion(DeletionMessage),
    EditMessage(EditMessage),
    CreatePoll(CreatePollMessage),
    VotePoll(VotePollMessage),
    Pin(PinMessage),
    Unpin(UnpinMessage),
    TerminatePoll(TerminatePollMessage),
    GroupChange(GroupChangeMessage),
    // TODO: How do we receive that? With a changed profile_key field maybe, or with the `flags` fiel?
    ContactChange(ContactChangeMessage),
    SessionReset,
    ExpirationTimerUpdate(ExpirationTimerUpdateMessage),
    Story(StoryMessage),
    Call(CallMessage),
    Payment(PaymentMessage),
    Null,

    // TODO: 
    // - required_protocol_version
    // - group_call_update
    // - story_context
    // - gift_badge
}

pub struct TextMessage {
    pub body: Option<String>,
    pub attachments: Vec<AttachmentPointer>,
    pub quote: Option<Quote>,
    pub body_ranges: Vec<BodyRange>,
    pub previews: Vec<Preview>,
    pub contacts: Vec<ContactInformation>,
    pub is_view_once: bool,
}


pub struct Quote {
    pub quoted_message: MessageId,
    pub body: Option<String>,
    pub attachments: Vec<AttachmentPointer>,
    pub body_ranges: Vec<BodyRange>,
    // TODO: type?
}

pub struct ReactionMessage {
    pub reaction_to_message: MessageId,
    pub emoji: String,
    pub operation: ReactionOperation,
}

pub enum ReactionOperation {
    Add,
    Remove,
}

pub struct ReceiptMessage {
    pub message_id: MessageId,
    pub kind: ReceiptKind,
}

pub enum ReceiptKind {
    Delivery,
    Read,
    Viewed,
}

pub struct TypingMessage {
    pub thread: ThreadId,
    pub action: TypingAction
}

pub enum TypingAction {
    Started,
    Stopped
}

pub struct DeletionMessage {
    pub deleted_message: MessageId,
}

pub struct EditMessage {
    pub edited_message: MessageId,
    pub updated: TextMessage,
}

pub struct CreatePollMessage {
    pub question: String,
    pub options: Vec<String>,
    pub allow_multiple: bool,
}

pub struct VotePollMessage {
    pub vote_for: MessageId,
    pub options: Vec<u32>,
    // TODO: vote_count?
}

pub struct TerminatePollMessage {
    pub poll: MessageId,
}

pub struct PinMessage {
    pub message: MessageId,
    pub duration: Option<PinDuration>,
}

pub enum PinDuration {
    Forever,
    Limited(Duration),
}

pub struct UnpinMessage {
    pub message: MessageId,
}

pub struct ContactChangeMessage {
    // TODO
}

pub struct GroupChangeMessage {
    // TODO
}

pub struct ExpirationTimerUpdateMessage {
    pub new_timer: Timer,
}

pub struct StoryMessage {
    // TODO
}

pub struct CallMessage {
    // TODO
}

pub struct PaymentMessage {
    // TODO
}
