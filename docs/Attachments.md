# `nvelope` - Attachments

Attachments are defined in [BCR-2023-006](https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2023-006-envelope-attachment.md). They are a standardized way to add discoverable third-party data to an envelope.

Attachments are assertions that can be built up by using various invocations of the `nvelope` command line tool, but since they have a specific format, the `nvelope` tool provides shortcuts for creating and working with them.

## Attachment Subcommands

```bash
👉
nvelope attachment --help
```

```
👈
Work with the envelope's attachments

Usage: nvelope attachment <COMMAND>

Commands:
  add          Add an assertion to the given envelope
  all          Retrieve all the envelope's assertions
  at           Get the attachment at the specified index
  conforms-to  Get the optional conformance of the attachment
  count        Print the count of the envelope's assertions
  create       Create an attachment
  payload      Get the payload of the attachment
  vendor       Get the vendor of the attachment
  find         Retrieve attachments having the specified attributes
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Creating an Attachment

The `attachment create` command lets you create an attachment by specifying its fields: `vendor`, `conforms-to` (optional), and `payload`.

```bash
👉
nvelope attachment create --help
```

```
👈
Create an attachment

Usage: nvelope attachment create [OPTIONS] <VENDOR> [PAYLOAD]

Arguments:
  <VENDOR>
          The vendor of the attachment. Usually a reverse domain name

  [PAYLOAD]
          The payload of the attachment. Entirely defined by the vendor.

          If not supplied, it is read from stdin.

Options:
  -c, --conforms-to <CONFORMS_TO>
          An optional `conforms-to` value of the attachment. Usually a URI

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

But first we need an envelope that is our attachment "payload", i.e., the vendor-specific data that we want to attach to an envelope. This can any possible envelope of arbitrary complexity, but here we'll just use a simple string:

```bash
👉
PAYLOAD_ENVELOPE=`nvelope subject type string "this-is-the-payload"`
```

Now we create our attachment with our payload, and specify the vendor and conformance strings:

```bash
👉
VENDOR="com.example"
CONFORMS_TO="https://example.com/attachment"
ATTACHMENT=`nvelope attachment create $VENDOR --conforms-to $CONFORMS_TO $PAYLOAD_ENVELOPE`
```

Here's what our attachment looks like in envelope notation:

```bash
👉
nvelope format $ATTACHMENT
```

```
👈
'attachment': {
    "this-is-the-payload"
} [
    'conformsTo': "https://example.com/attachment"
    'vendor': "com.example"
]
```

Notice that attachments are "bare assertions," i.e., assertions without a subject. Recall that assertions have a predicate and an object: in this case the predicate is the known value `'attachment'` and the object is the payload, which is wrapped and then has its own `'vendor'` and `'conformsTo'` assertions added to it.

## Querying an Attachment

Now that we have an attachment, we can query it for its vendor:

```bash
👉
nvelope attachment vendor $ATTACHMENT
```

```
👈
com.example
```

Or for its conformance:

```bash
👉
nvelope attachment conforms-to $ATTACHMENT
```

```
👈
https://example.com/attachment
```

Or for its payload:

```bash
👉
nvelope attachment payload $ATTACHMENT
```

```
👈
ur:envelope/tpcsjkjyisinjkdpinjkdpjyisihdpjohskkjzjlhsiefgjtesdk
```

Note that since the payload is itself an envelope which can be arbitrarily complex, it is returned as a UR. So we can take the result of the above command and extract its subject as a string:

```bash
👉
nvelope extract string `nvelope attachment payload $ATTACHMENT`
```

```
👈
this-is-the-payload
```

## Adding an Attachment

Now that we have an attachment, let's create a simple envelope to add it to:

```bash
👉
ENVELOPE=`nvelope subject type string "this-is-the-envelope"`
```

Since we already have an attachment, we can add it to our envelope using the `attachment add envelope` command:

```bash
👉
ENVELOPE_WITH_ATTACHMENT=`nvelope attachment add envelope $ATTACHMENT $ENVELOPE`

nvelope format $ENVELOPE_WITH_ATTACHMENT
```

```
👈
"this-is-the-envelope" [
    'attachment': {
        "this-is-the-payload"
    } [
        'conformsTo': "https://example.com/attachment"
        'vendor': "com.example"
    ]
]
```

## Multiple Attachments

Let's say the vendor releases a new version of the attachment spec. But for backwards compatibility, they want to support both the old and the new version. So they create a new attachment with the new version, and add it to the envelope. we start by creating the version 2 payload:

```bash
👉
PAYLOAD_ENVELOPE_V2=`nvelope subject type string "this-is-the-payload-v2"`
```

In the previous example we created the attachment and then added it to the envelope, but we can also do it in one step by using the `attachment add components` command:

```bash
👉
ENVELOPE_WITH_TWO_ATTACHMENTS=`nvelope attachment add components "com.example" --conforms-to "https://example.com/attachment-v2" $PAYLOAD_ENVELOPE_V2 $ENVELOPE_WITH_ATTACHMENT`

nvelope format $ENVELOPE_WITH_TWO_ATTACHMENTS
```

```
👈
"this-is-the-envelope" [
    'attachment': {
        "this-is-the-payload"
    } [
        'conformsTo': "https://example.com/attachment"
        'vendor': "com.example"
    ]
    'attachment': {
        "this-is-the-payload-v2"
    } [
        'conformsTo': "https://example.com/attachment-v2"
        'vendor': "com.example"
    ]
]
```

## Enumerating Attachments

The `count`, `all`, and `at` commands let you enumerate the attachments in an envelope:

```bash
👉
nvelope attachment count $ENVELOPE_WITH_TWO_ATTACHMENTS
```

```
👈
2
```

```bash
👉
nvelope attachment all $ENVELOPE_WITH_TWO_ATTACHMENTS
```

```
👈
ur:envelope/oycseylstpsptpcsjkjyisinjkdpinjkdpjyisihdpjohskkjzjlhsieoycseetpcsksckisjyjyjojkftdldlihkshsjnjojzihdmiajljndlhsjyjyhsiaisjnihjtjyoycseotpcsjeiajljndmihkshsjnjojzihzozmhhao
ur:envelope/oycseylstpsptpcskojyisinjkdpinjkdpjyisihdpjohskkjzjlhsiedpkoeyoycseetpcsksclisjyjyjojkftdldlihkshsjnjojzihdmiajljndlhsjyjyhsiaisjnihjtjydpkoeyoycseotpcsjeiajljndmihkshsjnjojzihjtlflpst
```

```bash
👉
nvelope attachment at 0 $ENVELOPE_WITH_TWO_ATTACHMENTS
```

```
👈
ur:envelope/oycseylstpsptpcsjkjyisinjkdpinjkdpjyisihdpjohskkjzjlhsieoycseetpcsksckisjyjyjojkftdldlihkshsjnjojzihdmiajljndlhsjyjyhsiaisjnihjtjyoycseotpcsjeiajljndmihkshsjnjojzihzozmhhao
```

## Finding Attachments

The `find` command lets you find attachments that match a given set of attributes. In these examples we pipe the results to `wc -l` to count the number of attachments that match the given attributes.

There are two attachments in our envelope, and both were added by the same vendor, so we can find them both by specifying the vendor:

```bash
👉
nvelope attachment find --vendor "com.example" $ENVELOPE_WITH_TWO_ATTACHMENTS | wc -l
```

```
👈
2
```

Each of these two attachments have different conformance URIs, so we can just find the version 2 attachment by specifying its conformance URI:

```bash
👉
nvelope attachment find --conforms-to "https://example.com/attachment-v2" $ENVELOPE_WITH_TWO_ATTACHMENTS | wc -l
```

```
👈
1
```
