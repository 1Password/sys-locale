#import <Foundation/NSLocale.h>
#import <Foundation/Foundation.h>

const char* apple_fetch_locale() {
    NSString* language_ptr = NSLocale.preferredLanguages.firstObject;

    if (language_ptr == nil) {
        return nil;
    }

    const char* string_ptr = [language_ptr UTF8String];

    char* language = (char*) malloc(strlen(string_ptr) + 1);

    if (language == nil) {
        return nil;
    }

    strcpy(language, string_ptr);

    return language;
}
