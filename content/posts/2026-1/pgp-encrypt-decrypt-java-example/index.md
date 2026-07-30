---
title: "PGP 加解密及 Java 代码演示"
url: /pgp-encrypt-decrypt-java-example/
date: 2026-07-30T14:40:20-05:00
featured: false
draft: false
type: post
toc: false
# menu: main
usePageBundles: true
thumbnail: "../images/logos/java-logo.png"
categories:
  - Java
tags: 
  - PGP
comment: true
codeMaxLines: 50
showLastmod: true
lastmod:
---

说起 PGP(Pretty Good Privacy) 的工作原理，类似早期的 TLS/HTTPS 的 RSA 密钥交换，只不过它不依赖于第三方的 CA，而是直接在两端以信任的方式直接交换了公钥和私钥。
它是基于以下几点进行工作的

- 用工具(如 PGPTool 或 GnuPG) 生成公钥和私钥对，公钥发给加密方，私钥保存在解密方
- 公钥加密的数据只能用相应的私钥解密，这是非对称加密
- 由于非对称加密解密的计算量较大，通常用一个较小随机 Key 对数据进行对称加密，而只对 Key 用公钥进行非对称加密，再把对称加密的数据与非对称加密的 Key
  发给解密方，在解密方用私钥解密出 Key, 再对数据进行解密。-- 这称之为混合加密，也是目前 HTTPS 的工作方式
- 另外，在只持有公钥的一方可以对使用私钥加密的数据进行签名验证，验证数据是否是由持有私钥的一方签发的，这就是数字签名的工作原理
- PGP 也会对生成的私钥用一个口令对其进行加密，防止私钥被盗时进一步保护
- 只用私钥可以做加解密与签名验证，如加密的数据能用相同的私钥解密，私钥签名再私钥验证。但交换私钥是不安全的，所以才有了可公开的公钥，非对称加密

下面用 GnuPG 和 Java 的 [Bouncy Castle 包](https://www.bouncycastle.org/download/bouncy-castle-java/) 进行演示加解密的过程。<!--more-->

在 mac OS 下安装 pgp 命令

> brew install gnupg

### 用 pgp 命令生成公私钥对

用 `gpg --full-generate-key` 命令，提示选择时选择默认选项，如

- ECS (sign and encrypt) *default*
- Curve 25519 *default*
- 有效期输入 1m
- 输入备注，名称和邮箱
- 最后输入保护私钥的口令，在弹出密码输入窗口这里输入 `password123`

```bash
gpg --full-generate-key
gpg (GnuPG) 2.5.21; Copyright (C) 2026 g10 Code GmbH
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Please select what kind of key you want:
   (1) RSA and RSA
   (2) DSA and Elgamal
   (3) DSA (sign only)
   (4) RSA (sign only)
   (9) ECC (sign and encrypt) *default*
  (10) ECC (sign only)
  (14) Existing key from card
  (16) ECC and Kyber
Your selection?
Please select which elliptic curve you want:
   (1) Curve 25519 *default*
   (4) NIST P-384
   (6) Brainpool P-256
Your selection?
Please specify how long the key should be valid.
         0 = key does not expire
      <n>  = key expires in n days
      <n>w = key expires in n weeks
      <n>m = key expires in n months
      <n>y = key expires in n years
Key is valid for? (0) 1m
Key expires at Sat Aug 29 11:13:11 2026 CDT
Is this correct? (y/N) y

GnuPG needs to construct a user ID to identify your key.

Real name: Yanbin
Email address: yabqiu@gmail.com
Comment:
You selected this USER-ID:
    "Yanbin <yabqiu@gmail.com>"

Change (N)ame, (C)omment, (E)mail or (O)kay/(Q)uit? O
We need to generate a lot of random bytes. It is a good idea to perform
some other action (type on the keyboard, move the mouse, utilize the
disks) during the prime generation; this gives the random number
generator a better chance to gain enough entropy.
We need to generate a lot of random bytes. It is a good idea to perform
some other action (type on the keyboard, move the mouse, utilize the
disks) during the prime generation; this gives the random number
generator a better chance to gain enough entropy.
gpg: revocation certificate stored as '/Users/yanbin/.gnupg/openpgp-revocs.d/F5209BA90BC24513339DCE0341DF8CD698924E33.rev'
public and secret key created and signed.

pub   ed25519 2026-07-30 [SC] [expires: 2026-08-29]
      F5209BA90BC24513339DCE0341DF8CD698924E33
uid                      Yanbin <yabqiu@gmail.com>
sub   cv25519 2026-07-30 [E] [expires: 2026-08-29]
      2FAEEA94A20F85C9BACB97F14929F90C8FFBB71C
```

生成的私钥保存在 `./.gnupg/` 目录下，查看公钥和私钥的命令分别为

```bash
gpg --list-keys
gpg --list-secret-keys
```

要在别处用的话需要导公钥和私钥，导出公钥命令为

```bash
gpg --armor --export yabqiu@gmail.com > yanbin_public.asc
```

导出私钥的命令为

```bash
gpg --armor --export-secret-keys yabqiu@gmail.com > yanbin_private.asc
```

导出私钥时会提示输入相同的口令，这里是 `password123`.

完后，查看公私钥文件

```bash
ls -l
-rw-r--r--@ 1 yanbin  root   955 Jul 30 11:22 yanbin_private.asc
-rw-r--r--@ 1 yanbin  root   725 Jul 30 11:22 yanbin_public.asc
```

注意导出公私钥时邮箱要与生成密钥时的邮箱一致，否则什么也不能导出。

继续用 `gpg` 命令的话，完全可由它完成加密，解密，签名，验证签名的所有用工作，但我们下面要转入到 Java 代码中来。

### Java 使用 PGP 加密

首先创建一个 Maven 项目，并加入 org.bouncycastle 的两个依赖

```xml
<dependency>
    <groupId>org.bouncycastle</groupId>
    <artifactId>bcprov-jdk18on</artifactId>
    <version>1.85</version>
</dependency>
<dependency>
    <groupId>org.bouncycastle</groupId>
    <artifactId>bcpg-jdk18on</artifactId>
    <version>1.85</version>
</dependency>
```

`bcprov` 是核心加密库，`bcpg` 是 `OpenPGP` 实现模块。

把前面导出的公私钥文件拷贝到 Maven 项目的 `src/main/resources/` 目录中，以便后面用 `ClassLoader` 来读取。完整加密代码如下

```java
package blog.yanbin;

import org.bouncycastle.bcpg.ArmoredOutputStream;
import org.bouncycastle.bcpg.SymmetricKeyAlgorithmTags;
import org.bouncycastle.jce.provider.BouncyCastleProvider;
import org.bouncycastle.openpgp.*;
import org.bouncycastle.openpgp.operator.jcajce.JcaKeyFingerprintCalculator;
import org.bouncycastle.openpgp.operator.jcajce.JcePGPDataEncryptorBuilder;
import org.bouncycastle.openpgp.operator.jcajce.JcePublicKeyKeyEncryptionMethodGenerator;

import java.io.*;
import java.security.Security;
import java.util.Date;

public class PgpEncryptor {

  static {
    Security.addProvider(new BouncyCastleProvider()); // 不注册的话会得到执行错误：No such provider: BC
  }

  static byte[] encrypt(byte[] data) throws Exception {
    InputStream publicKeyInputStream = Thread.currentThread().getContextClassLoader().getResourceAsStream("yanbin_public.asc");
    PGPPublicKeyRingCollection pgpPublicKeyRings = new PGPPublicKeyRingCollection(
            PGPUtil.getDecoderStream(publicKeyInputStream), new JcaKeyFingerprintCalculator());
    PGPPublicKey publicKey = null;
    for (PGPPublicKeyRing ring : pgpPublicKeyRings) {
      for (PGPPublicKey key : ring) {
        if (key.isEncryptionKey()) {
          publicKey = key;
          break;
        }
      }
      if (publicKey != null) {
        break;
      }
    }
    if (publicKey == null) {
      throw new IllegalArgumentException("Can't find encryption key in key ring.");
    }
    JcePGPDataEncryptorBuilder encryptorBuilder =
            new JcePGPDataEncryptorBuilder(SymmetricKeyAlgorithmTags.AES_256)
                    .setWithIntegrityPacket(true)
                    .setSecureRandom(new java.security.SecureRandom())
                    .setProvider("BC");
    PGPEncryptedDataGenerator encGen = new PGPEncryptedDataGenerator(encryptorBuilder);
    encGen.addMethod(new JcePublicKeyKeyEncryptionMethodGenerator(publicKey).setProvider("BC"));

    ByteArrayOutputStream baos = new ByteArrayOutputStream();
    try (OutputStream out = new ArmoredOutputStream(baos);
         OutputStream encOut = encGen.open(out, new byte[1024 * 64]);
         OutputStream literalOut = new PGPLiteralDataGenerator().open(
                 encOut, PGPLiteralData.BINARY, PGPLiteralData.CONSOLE, data.length, new Date())) {
      literalOut.write(data);
    }
    return baos.toByteArray();
  }

  public static void main(String[] args) throws Exception {
    byte[] encrypted = encrypt("Hello, World!".getBytes());
    System.out.println(new String(encrypted));
  }
}
```

执行后输出为

```text
-----BEGIN PGP MESSAGE-----
Version: BCPG v1.85

wV4DSSn5DI/7txwSAQdAmN8CNPzTl/4Uz++29w7ler6LhHb7NeZuqGWdcJMoiHMw
kmYb304gNAx7GKQvHp0UXEWb7Fn2i6yYDG4HNTfdQEQaEiBVQ7pKuZIT0Mmo8nIh
0kYB3irP9q1dxIMoxEl9rLQ4GjuKzeG4ezqylLgZNmW5YYqVM4ZD9R1DzgI1EmtY
xHFhpmi2S8XHlpp+GbxrA+aW2lxM9edE
=jm4E
-----END PGP MESSAGE-----
```

这就是 `Hello, World!` 加密后的 PGP 密文，包含用随机 Key 对数据进行的对称加密，以及用公钥加密的随机 Key。由于是随机的 Key, 所以每次加密后的数据是不一样的。

### Java 使用 PGP 解密

直接解密上面生成的密文，完整代码如下

```java
package blog.yanbin;

import org.bouncycastle.jce.provider.BouncyCastleProvider;
import org.bouncycastle.openpgp.*;
import org.bouncycastle.openpgp.jcajce.JcaPGPObjectFactory;
import org.bouncycastle.openpgp.operator.jcajce.JcaKeyFingerprintCalculator;
import org.bouncycastle.openpgp.operator.jcajce.JcePBESecretKeyDecryptorBuilder;
import org.bouncycastle.openpgp.operator.jcajce.JcePublicKeyDataDecryptorFactoryBuilder;

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.InputStream;
import java.security.Security;
import java.util.Iterator;

public class PgpDecryptor {

    static {
        Security.addProvider(new BouncyCastleProvider()); // 不注册的话会得到执行错误：No such provider: BC
    }

    private static byte[] decrypt(byte[] data, String passphrase) throws Exception {
        InputStream privateKeyInputStream = Thread.currentThread().getContextClassLoader().getResourceAsStream("yanbin_private.asc");
        PGPSecretKeyRingCollection secretKeyRings = new PGPSecretKeyRingCollection(
            PGPUtil.getDecoderStream(privateKeyInputStream),
            new JcaKeyFingerprintCalculator());

        InputStream decoderStream = PGPUtil.getDecoderStream(new ByteArrayInputStream(data));
        PGPObjectFactory pgpObjectFactory = new JcaPGPObjectFactory(decoderStream);
        Object o = pgpObjectFactory.nextObject();
        PGPEncryptedDataList enc = (o instanceof PGPEncryptedDataList) ? (PGPEncryptedDataList) o :
            (PGPEncryptedDataList) pgpObjectFactory.nextObject();
        Iterator<PGPEncryptedData> encryptedDataObjects = enc.getEncryptedDataObjects();

        PGPPrivateKey privateKey = null;
        PGPPublicKeyEncryptedData encData = null;
        while (privateKey == null && encryptedDataObjects.hasNext()) {
            Object next = encryptedDataObjects.next();
            if (!(next instanceof PGPPublicKeyEncryptedData)) continue;
            encData = (PGPPublicKeyEncryptedData) next;

            PGPSecretKey secretKey = secretKeyRings.getSecretKey(encData.getKeyID());
            if (secretKey != null) {
                privateKey = secretKey.extractPrivateKey(
                    new JcePBESecretKeyDecryptorBuilder()
                        .setProvider("BC")
                        .build(passphrase.toCharArray()));
            }
        }

        InputStream clear = encData.getDataStream(
            new JcePublicKeyDataDecryptorFactoryBuilder().setProvider("BC").build(privateKey));

        PGPObjectFactory plainFact = new JcaPGPObjectFactory(clear);
        Object message = plainFact.nextObject();

        if (message instanceof PGPCompressedData cData) {
            PGPObjectFactory compFact = new JcaPGPObjectFactory(cData.getDataStream());
            message = compFact.nextObject();
        }

        if (message instanceof PGPLiteralData ld) {
            try (InputStream unc = ld.getInputStream();
                ByteArrayOutputStream out = new ByteArrayOutputStream()) {
                unc.transferTo(out);
                return out.toByteArray();
            }
        } else {
            throw new PGPException("unknown message type: " + message.getClass());
        }
    }

    public static void main(String[] args) throws Exception {
        byte[] encrypted = """
            -----BEGIN PGP MESSAGE-----
            Version: BCPG v1.85
            
            wV4DSSn5DI/7txwSAQdAmN8CNPzTl/4Uz++29w7ler6LhHb7NeZuqGWdcJMoiHMw
            kmYb304gNAx7GKQvHp0UXEWb7Fn2i6yYDG4HNTfdQEQaEiBVQ7pKuZIT0Mmo8nIh
            0kYB3irP9q1dxIMoxEl9rLQ4GjuKzeG4ezqylLgZNmW5YYqVM4ZD9R1DzgI1EmtY
            xHFhpmi2S8XHlpp+GbxrA+aW2lxM9edE
            =jm4E
            -----END PGP MESSAGE-----
            """.getBytes();
        byte[] decrypted = decrypt(encrypted, "password123");
        System.out.println(new String(decrypted));
    }
}
```

成功解密，输出为

> Hello, World!

解密的过程就是从加密数据中分离出两部分

1. 对称加密的数据
2. 非对称加密的 Key

然后用私钥解密出 Key, 然后使用该 Key 对数据进行解密。

注意到解密代码中有个判断是否为 `PGPCompressedData`

```java
    if (message instanceof PGPCompressedData cData) {
        PGPObjectFactory compFact = new JcaPGPObjectFactory(cData.getDataStream());
        message = compFact.nextObject();
    }
```

由于加密时没有使用数据压缩，所以这里的 `message` 不会是 `PGPCompressedData` 类型。

### 在加密时使用数据压缩

只需要在 `PgpEncryptor` 类中，把

```java
    try (OutputStream out = new ArmoredOutputStream(baos);
         OutputStream encOut = encGen.open(out, new byte[1024 * 64]);
         OutputStream literalOut = new PGPLiteralDataGenerator().open(
                 encOut, PGPLiteralData.BINARY, PGPLiteralData.CONSOLE, data.length, new Date())) {
         literalOut.write(data);
    }
```

改为

```java
        try (OutputStream out = new ArmoredOutputStream(baos);
            OutputStream encOut = encGen.open(out, new byte[1024 * 64]);
            OutputStream compOut = new PGPCompressedDataGenerator(PGPCompressedDataGenerator.ZIP).open(encOut);
            OutputStream literalOut = new PGPLiteralDataGenerator().open(
            compOut, PGPLiteralData.BINARY, PGPLiteralData.CONSOLE, data.length, new Date())) {
            literalOut.write(data);
        }
```

这时候加密前就会对数据进行压缩，解密时就会进入到 `PGPCompressedData` 的判断分支。对于小数据 `Hello World!` 启用压缩不会有什么改观，
甚至大小还会略大。

在使用了压缩后 `PgpEncryptor` 得到的结果是

```text
-----BEGIN PGP MESSAGE-----
Version: BCPG v1.85

wV4DSSn5DI/7txwSAQdAopwN3hVpbGkJ4MmDHBEYQiD0HzM4b7ecy+hQTzqt8W8w
jPgJnyQ70kVLurDz7Ccd10ncN+KTFBzGSayHnfAZeE2tdbqQFFQE6B1/0fSyNJQv
0koBb7Eb9mhLrVMIK/1Txn0NEomAm8NAOm+6z0soft0RVbILZQPdDLkG9xvd/yOz
El0AA85YvBE6Hf7etQWrVSBaxpzC9Auf5DTXkA==
=FltD
-----END PGP MESSAGE-----
```

其实我们在使用 PGP 加密之前使用 JDK 的 API 对输入数据自行压缩就用不着 Bouncy Castle 的 `PGPCompressedDataGenerator` 了。

### 加密时不使用 ArmoredOutputStream

`ArmoredOutputStream` 的功能是让生成的密文形式为 Base64 文本格式，像前面的

```text
-----BEGIN PGP MESSAGE-----
Version: BCPG v1.85

<base64 text>
-----END PGP MESSAGE-----
```

我们也可以去掉 `ArmoredOutputStream` 这一层，类似的，

```java
    try (OutputStream out = new ArmoredOutputStream(baos);
         OutputStream encOut = encGen.open(out, new byte[1024 * 64]);
         OutputStream literalOut = new PGPLiteralDataGenerator().open(
                 encOut, PGPLiteralData.BINARY, PGPLiteralData.CONSOLE, data.length, new Date())) {
         literalOut.write(data);
    }
```

改为

```java
        try (OutputStream encOut = encGen.open(baos, new byte[1024 * 64]);
            OutputStream compOut = new PGPCompressedDataGenerator(PGPCompressedDataGenerator.ZIP).open(encOut);
            OutputStream literalOut = new PGPLiteralDataGenerator().open(
              compOut, PGPLiteralData.BINARY, PGPLiteralData.CONSOLE, data.length, new Date())) {
            literalOut.write(data);
        }
```

这时候生成密文转成 String 就无法阅读了

```java
System.out.print(encrypt("Hello, World!".getBytes()));
```

> �^I)����@*���{�+���;��fRw�\-d7A��f�%L0�n�I��k��Eu�n���61��h��ŗv;�H��ųƿ��˅��}�JC{$�T��h���?�����GO��m�N}e������g�>!�fL�Ά6��������5�WO�$���F�F�

但把密文字节传递给解密代码是没问题的

```java
    byte[] encrypted  = encrypt("Hello World!".getBytes());
    byte[] decrypted = decrypt(encrypted, "password123");
    System.out.println(new String(decrypted));
```

同样能还原出

> Hello World!

另外，使用私钥签名，公钥验证签名的代码这里不予涉及。

### 观察用来加解密的 Key

从 PGP 的加密过程，我们知道 PGP 在加密时随机生成一个密钥，用该密钥对数据进行加密，然后用公钥对该密钥加密后发送给解密方(持有私钥，
可用私钥对非对称加密的密钥进行解密)，那我们能看看以上代码生成的密钥。

在 PgpEncryptor 类中适当位置加入代码

```java
encGen.setSessionKeyExtractionCallback(sessionKey ->
    System.out.printf("Encryption - Algorithm: %s, Session key: %s%n", sessionKey.getAlgorithm(), Arrays.toString(sessionKey.getKey())));
```

同时在 PgpDecryptor 类中加入代码

```java
PGPSessionKey sessionKey = encData.getSessionKey(decryptorFactory);
System.out.printf("Decryption - Algorithm: %s, Session key: %s%n", sessionKey.getAlgorithm(), Arrays.toString(sessionKey.getKey()));
```

而后我们执行 `PgpDecrptor` 就会一同输出

```text
Encryption - Algorithm: 9, Session key: [-110, -37, -73, 93, -29, 56, -97, -38, 37, -98, 32, 13, 123, -78, 9, 19, -8, -4, -55, -71, 97, 122, -111, 12, 94, 94, -118, -18, 78, -38, 48, 27]
Decryption - Algorithm: 9, Session key: [-110, -37, -73, 93, -29, 56, -97, -38, 37, -98, 32, 13, 123, -78, 9, 19, -8, -4, -55, -71, 97, 122, -111, 12, 94, 94, -118, -18, 78, -38, 48, 27]
```

可以发现在解密时正确的还原出了用于加密的密钥，以及对数据加密时相应的算法。

### 用 Python 还是简单的多

用 Java 代码进行 PGP 加解密的过程比预想的明显示要复杂，我们不妨看看隔壁 Python 的类似实现

如使用库 `python-gnupg`, 解密的代码如下

```python
import gnupg

gpg = gnupg.GPG()

with open("yanbin_private.asc", "r") as f:
    gpg.import_keys(f.read())

decrypted = gpg.decrypt("""
-----BEGIN PGP MESSAGE-----
Version: BCPG v1.85

wV4DSSn5DI/7txwSAQdAopwN3hVpbGkJ4MmDHBEYQiD0HzM4b7ecy+hQTzqt8W8w
jPgJnyQ70kVLurDz7Ccd10ncN+KTFBzGSayHnfAZeE2tdbqQFFQE6B1/0fSyNJQv
0koBb7Eb9mhLrVMIK/1Txn0NEomAm8NAOm+6z0soft0RVbILZQPdDLkG9xvd/yOz
El0AA85YvBE6Hf7etQWrVSBaxpzC9Auf5DTXkA==
=FltD
-----END PGP MESSAGE-----
""", passphrase="password123")

print(decrypted)
```

输出

> Hello, World!

加密的代码也很简单

```python
with open("yanbin_public.asc", "r") as f:
    import_result = gpg.import_keys(f.read())

status = gpg.encrypt(
    "Hello World!",
    recipients=import_result.fingerprints[0:],
    always_trust=True
)
encrypted = status.data
print(encrypted.decode())
```

输出为

```text
-----BEGIN PGP MESSAGE-----

hF4DSSn5DI/7txwSAQdAMeC8iDHl5Es82WCmRsZL/tP74KFZY12/E/fXeWJWdS8w
yeADHLp+KNBo2dt+LFzjSsfWPvTIwzVp2mA8gPxBYm0NSuf0rk0YbxpeLNSjcMzP
1FEBCQIQwV3r9xxi35qxm1r+n6qpMPX9wSEPVccBt1SgbjByAKfsZRlBDkY3QSxy
J/h5JtKNeHhM9xSz3LHcN0yCbAdA2Ci4NzdS/5LDk3tV/Pk=
=Paj8
-----END PGP MESSAGE-----
```

能用 Python 还是选择 Python 吧。