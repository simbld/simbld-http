# ⚙️ Http's Code Families

`Official` HTTP codes.
*Unofficial* HTTP codes.

---

### 📚 Informational responses

`ContinueRequest` = **100**\
`SwitchingProtocols` = **101**

*Processing* = **102**\
*EarlyHints* = **103**\
*ConnectionResetByPeer* = **104**\
*NameNotResolved* = **105**\
*NoResponse* = **106**\
*RetryWith* = **107**\
*ResponseIsStale* = **108**\
*RevalidationFailed* = **109**

---

### 📚 Successful responses

`Ok` = **200**\
`Created` = **201**\
`Accepted` = **202**\
`NonAuthoritativeInformation` = **203**\
`NoContent` = **204**\
`ResetContent` = **205**\
`PartialContent` = **206**

*MultiStatus* = **207**\
*AlreadyReported* = **208**\
*ContentDifferent* = **210**\
*ContentLocation* = **211**\
*ObjectData* = **212**\
*MultipleResourceInstances* = **213**\
*TransformApplied* = **214**\
*ContentDeleted* = **215**\
*IMUsedPostRequest* = **216**\
*DeltaEncodingApplied* = **217**\
*ThisIsFine* = **218**\
*ContentTransferred* = **219**\
*LoadBalancerStarted* = **220**\
*LoadBalancerEnded* = **221**\
*AuthenticationSuccessful* = **222**\
*IMUsedGetRequest* = **226**\
*LowOnStorageSpace* = **250**\
*EntityRecognizedNotProcessable* = **252**\
*ResourceAccessedLocked* = **253**\
*MethodNotFound* = **254**\
*ExtendedCode* = **255**\
*MiscellaneousPersistentWarningStart* = **299**

---

### 📚 Redirection responses

`MultipleChoices` = **300**\
`MovedPermanently` = **301**\
`Found` = **302**\
`SeeOther` = **303**\
`NotModified` = **304**\
`UseProxy` = **305**\
`SwitchProxy` = **306**\
`TemporaryRedirect` = **307**\
`PermanentRedirect` = **308**

*TooManyRedirects* = **310**\
*RedirectMethod* = **311**\
*Unassigned* = **312**\
*MovedPermanentlyRedirected* = **321**\
*MovedTemporarilyRedirected* = **322**\
*SeeOtherRedirected* = **323**\
*NotModifiedRedirected* = **324**\
*UseProxyRedirected* = **325**\
*UnusedRedirected* = **326**\
*TemporaryRedirectRedirected* = **327**\
*PermanentRedirected* = **328**\
*TooManyRedirectsRedirected* = **329**\
*RedirectMethodRedirected* = **330**\
*UserNameOkPasswordNeeded* = **331**\
*NoNeedAccountForLogin* = **332**\
*SessionKeyNotPresentInHeader* = **333**\
*SessionKeyPresentAndNotDecryptableParsable* = **334**\
*ServerIsUnwillingToProcessTheRequest* = **335**\
*ChallengeResponseAuthenticationOk* = **336**\
*ChallengeResponseAuthenticationFailed* = **337**\
*LengthRequired* = **342**\
*PreconditionFailed* = **343**\
*RequestEntityTooLarge* = **344**\
*UnsupportedMediaType* = **346**\
*RequestedRangeNotSatisfiable* = **347**\
*ExpectationFailed* = **348**\
*ImATeapot* = **349**\
*ErrorAccessingURL* = **350**\
*TriggerNotFound* = **351**\
*AccessDenied* = **352**\
*ConditionFailed* = **353**\
*MandatoryParameterIsNull* = **354**\
*TheParameterDoesNotExist* = **355**\
*DataBLOBShouldNotBeNullForPostMethod* = **356**

---

### 📚 Client errors

`BadRequest` = **400**\
`Unauthorized` = **401**\
`PaymentRequired` = **402**\
`Forbidden` = **403**\
`NotFound` = **404**\
`MethodNotAllowed` = **405**\
`NotAcceptable` = **406**\
`ProxyAuthenticationRequired` = **407**\
`RequestTimeout` = **408**\
`Conflict` = **409**\
`Gone` = **410**\
`LengthRequired` = **411**\
`PreconditionFailed` = **412**\
`PayloadTooLarge` = **413**\
`RequestUriTooLong` = **414**\
`UnsupportedMediaType` = **415**\
`RequestedRangeUnsatisfiable` = **416**\
`ExpectationFailed` = **417**\
`ImATeapot` = **418**

*PageExpired* = **419**\
*MethodFailure* = **420**

`MisdirectedRequest` = **421**\
`UnprocessableEntity` = **422**

*LockedTemporarilyUnavailable* = **423**\
*FailedDependency* = **424**\
*TooEarly* = **425**

`UpgradeRequired` = **426**

*PreconditionRequired* = **428**\
*TooManyRequests* = **429**\
*RequestHeaderFieldsTooLarge* = **430**\
*LoginRequired* = **432**\
*OriginError* = **433**\
*DestinationError* = **434**\
*TooLarge* = **435**\
*SSLCertificateError* = **436**\
*SSLCertificateRequired* = **437**\
*NoCertificate* = **438**\
*LoginTimeout* = **440**\
*OverDataQuota* = **441**\
*NoResponse* = **444**\
*RetryWith* = **449**\
*BlockedByWindowsParentalControls* = **450**\
*UnavailableForLegalReasons* = **451**\
*TooManyRecipients* = **452**\
*MethodNotValidInThisState* = **455**\
*UnrecoverableError* = **456**\
*ClientClosedConnexionPrematurely* = **460**\
*TooManyForwardedIPAddresses* = **463**\
*InternetSecurityError* = **467**\
*TemporaryUnavailable* = **480**\
*RequestHeaderTooLarge* = **494**\
*CertError* = **495**\
*NoCert* = **496**\
*HTTPToHTTPS* = **497**\
*InvalidToken* = **498**\
*ClientClosedRequest* = **499**

---

### 📚 Server errors

`InternalServerError` = **500**\
`NotImplemented` = **501**\
`BadGateway` = **502**\
`ServiceUnavailable` = **503**\
`GatewayTimeout` = **504**\
`HTTPVersionNotSupported` = **505**\
*VariantAlsoNegotiates* = **506**\
*InsufficientStorage* = **507**\
*LoopDetected* = **508**\
*BandwidthLimitExceeded* = **509**\
*NotExtended* = **510**\
*NetworkAuthenticationRequired* = **511**\
*UnknownError* = **520**\
*WebServerIsDown* = **521**\
*ConnectionTimedOut* = **522**\
*OriginIsUnreachable* = **523**\
*TimeoutOccurred* = **524**\
*SSLHandshakeFailed* = **525**\
*InvalidSSLCertificate* = **526**\
*RailgunError* = **527**\
*SiteIsOverloaded* = **529**\
*SiteIsFrozen* = **530**\
*OriginDNSError* = **531**\
*NoSiteDetected* = **561**\
*NetworkReadTimeoutError* = **598**\
*NetworkConnectTimeoutError* = **599**\

---

### 📚 Service responses

*ReadingError* = **611**\
*ConnectionError* = **612**\
*ReadingTimeExpired* = **613**\
*SSLHandshakeFailed* = **614**\
*AnotherReadingError* = **615**\
*FBAAnomaly* = **616**\
*CodingError* = **617**\
*RedirectWithoutRedirectURL* = **618**\
*DNSLookupFailed* = **680**\
*SyntacticallyIncorrectURL* = **690**\
*LostConnection* = **691**\
*WriteTimeout* = **692**\
*SelectionFailed* = **693**\
*WriteError* = **694**\
*IncompleteBlockHeader* = **695**\
*UnexpectedError* = **699**\

---

### 📚 Crawler responses

*ParsingErrorUnfinishedHeader* = **700**\
*ParsingErrorHeader* = **710**\
*ParsingErrorMissingHTTPCode* = **720**\
*ParsingErrorBody* = **730**\
*ExcludedByRobotsTxtFile* = **740**\
*RobotsTemporarilyUnavailable* = **741**\
*ExcludedByDefinitionOfExplorationSpace* = **760**\
*NotAllowedByLocalExplorationSpace* = **761**\
*IncorrectProtocolOrNonStandardSystemPort* = **770**\
*ExcludedByFileTypeExclusions* = **780**\
*InvalidCard* = **781**\
*CannotDisablePhysicalCard* = **782**\
*InvalidURL* = **786**\
*NoIndexMetaTag* = **2004**\
*ProgrammableRedirection* = **3020**\
*RedirectedToAnotherURL* = **3021**\

---

### 📚 Local responses

*ApprovedNoActionRequired* = **900**\
*Approved* = **901**\
*DuplicatedTransactionId* = **902**\
*ValidationErrorsProvided* = **903**\
*OperationNotAllowed* = **904**\
*OperationNotSupported* = **905**\
*TransactionTimeout* = **906**\
*AuthentificationFailed* = **907**\
*DoNotHonor* = **908**\
*InsufficientFunds* = **909**\
*IncorrectPIN* = **910**\
*InvalidTransaction* = **911**\
*InvalidAmount* = **912**\
*InvalidCardNumber* = **913**\
*InvalidCVV* = **914**\
*InvalidCardHolderName* = **915**\
*InvalidCardHolderLastName* = **916**\
*InvalidCardHolderFirstName* = **917**\
*InvalidCardHolderIdNumber* = **918**\
*InvalidCardHolderPhoneNumber* = **919**\
*CardAlreadyActive* = **920**\
*CardNotActive* = **921**\
*ExpiredCard* = **922**\
*LostCard* = **923**\
*StolenCard* = **924**\
*InvalidLastName* = **925**\
*InvalidFirstName* = **926**\
*InvalidIdNumber* = **927**\
*InvalidPhoneNumber* = **928**\
*InvalidEmail* = **929**\
*InvalidInitials* = **930**\
*InvalidAddress* = **931**\
*InvalidCity* = **932**\
*InvalidPostalCode* = **933**\
*InvalidCountry* = **934**\
*InvalidPassword* = **935**\
*InvalidUsername* = **936**\
*InvalidRole* = **937**\
*InvalidStatus* = **938**\
*InvalidDateOfBirth* = **939**\
*InvalidMajority* = **940**\
*InvalidMaritalStatus* = **941**\
*InvalidNationality* = **942**\
*InvalidLanguage* = **943**\
*InvalidTimeZone* = **945**\
*InvalidProfilePicture* = **946**\
*InvalidCoverPicture* = **947**\
*InvalidBio* = **948**\
*InvalidWebsite* = **949**\
*InvalidFacebook* = **950**\
*InvalidTwitter* = **951**\
*InvalidInstagram* = **952**\
*InvalidLinkedIn* = **953**\
*InvalidGitHub* = **954**\
*InvalidGitLab* = **955**\
*InvalidBitbucket* = **956**\
*InvalidGoogle* = **957**\
*InvalidYouTube* = **958**\
*InvalidTwitch* = **959**\
*InvalidDiscord* = **960**\
*InvalidSlack* = **961**\
*InvalidTelegram* = **962**\
*InvalidWhatsApp* = **963**\
*InvalidSkype* = **964**\
*InvalidSnapchat* = **965**\
*InvalidPinterest* = **966**\
*InvalidTumblr* = **967**\
*InvalidFlickr* = **968**\
*InvalidVimeo* = **969**\
*InvalidSoundCloud* = **970**\
*InvalidSpotify* = **971**\
*InvalidTikTok* = **972**\
*InvalidVine* = **973**\
*InvalidPeriscope* = **974**\
*InvalidExpirationDate* = **975**\
*SessionKeyNotPresentInHeader* = **976**\
*SessionKeyPresentAndNotDecryptableParsable* = **977**\
*ReferenceHasNoLinkedCards* = **978**\
*CardAlreadyLinkedToADifferentReference* = **979**\
*ExcludedByFileTypeExclusions* = **980**\
*InvalidCardInformation* = **981**\
*CannotDisablePhysicalCard* = **982**\
*MissingToken* = **983**\
*UserNotFound* = **984**\
*AlreadyExists* = **985**\
*DatabaseError* = **986**\
*HashingError* = **987**\
*InvalidLogin* = **988**\
*InvalidUser* = **989**\
*InvalidUserId* = **990**\
*InvalidUserRole* = **991**\
*InvalidCredentials* = **992**\
*UserAlreadyExists* = **993**\
*InvalidPseudonym* = **994**\
*InvalidTag* = **995**\
*InvalidAuthorizationCode* = **996**\
*RequestDenied* = **999**
