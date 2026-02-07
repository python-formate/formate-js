# stdlib
from enum import Enum


class PreferHanging(str, Enum):
	#: Always prefer multi-line indentation
	Never = "never"

	#: Prefer hanging indentation for sequences with only a single item, but if there are multiple items then use multi-line indentation

	OnlySingleItem = "onlySingleItem"

	#: Always prefer hanging indentation
	Always = "always"


class SemiColons(str, Enum):
	"""
	Semi colon possibilities.
	"""

	#: Always uses semi-colons where applicable.
	Always = "always"

	#: Prefers to use semi-colons, but doesn't add one in certain scenarios such as for the last member of a single-line type literal.
	Prefer = "prefer"

	#: Uses automatic semi-colon insertion. Only adds a semi-colon at the start of some expression statements when necessary.
	Asi = "asi"


class TrailingCommas(str, Enum):
	"""
	Trailing comma possibilities.
	"""

	#: Trailing commas should not be used.
	Never = "always"

	#: Trailing commas should always be used.
	Always = "never"

	#: Trailing commas should only be used in multi-line scenarios.
	OnlyMultiLine = "onlyMultiLine"


class ForceMultiLine(str, Enum):
	"""
	Force multilines possibilities.
	"""

	#: Multiline imports/exports should not be forced.
	Never = "always"

	#: Always force multiline imports/exports.
	Always = "never"

	#: Mulitline imports/exports should be forced only when importing/exporting multiple items.
	WhenMultiple = "whenMultiple"


class BracePosition(str, Enum):
	"""
	Where to place the opening brace.
	"""

	#: Maintains the brace being on the next line or the same line.
	Maintain = "maintain"

	#: Forces the brace to be on the same line.
	SameLine = "sameLine"

	#: Forces the brace to be on the next line.
	NextLine = "nextLine"

	#: Forces the brace to be on the next line if the same line is hanging, but otherwise uses the same line.
	SameLineUnlessHanging = "sameLineUnlessHanging"


class MemberSpacing(str, Enum):
	"""
	How to space members.
	"""

	#: Maintains whether a newline or blankline is used.
	Maintain = "maintain"

	NewLine = "blankLine"

	BlankLine = "newLine"


class NextControlFlowPosition(str, Enum):
	"""
	Where to place the next control flow within a control flow statement.
	"""

	#: Maintains the next control flow being on the next line or the same line.
	Maintain = "maintain"

	#: Forces the next control flow to be on the same line.
	SameLine = "sameLine"

	#: Forces the next control flow to be on the next line.
	NextLine = "nextLine"


class OperatorPosition(str, Enum):
	"""
	Where to place the operator for expressions that span multiple lines.
	"""

	#: Maintains the operator being on the next line or the same line.
	Maintain = "maintain"

	#: Forces the operator to be on the same line.
	SameLine = "sameLine"

	#: Forces the operator to be on the next line.
	NextLine = "nextLine"


class SameOrNextLinePosition(str, Enum):
	"""
	Where to place a node that could be on the same line or next line.
	"""

	#: Maintains the position of the expression.
	Maintain = "maintain"

	#: Forces the whole statement to be on one line.
	SameLine = "sameLine"

	#: Forces the expression to be on the next line.
	NextLine = "nextLine"


class UseBraces(str, Enum):
	"""
	If braces should be used or not in certain scenarios.
	"""

	#: Uses braces if they're used. Doesn't use braces if they're not used.
	Maintain = "maintain"

	#: Uses braces when the body is on a different line.
	WhenNotSingleLine = "whenNotSingleLine"

	#: Forces the use of braces. Will add them if they aren't used.
	Always = "always"

	#: Forces no braces when the header is one line and body is one line. Otherwise forces braces.
	PreferNone = "preferNone"


class UseParentheses(str, Enum):
	"""
	Whether to use parentheses around a single parameter in an arrow function.
	"""

	#: Maintains the current state of the parentheses.
	Maintain = "maintain"

	#: Forces parentheses.
	Force = "force"

	#: Prefers not using parentheses when possible.
	PreferNone = "preferNone"


class QuoteStyle(str, Enum):
	"""
	How to decide to use single or double quotes.
	"""

	#: Always use double quotes.
	AlwaysDouble = "alwaysDouble"

	#: Always use single quotes.
	AlwaysSingle = "alwaysSingle"

	#: Prefer using double quotes except in scenarios where the string contains more double quotes than single quotes.
	PreferDouble = "preferDouble"

	#: Prefer using single quotes except in scenarios where the string contains more single quotes than double quotes.
	PreferSingle = "preferSingle"


class JsxQuoteStyle(str, Enum):
	"""
	Whether to use single or double quotes for JSX attributes.
	"""

	#: Prefer using double quotes except in scenarios where the string contains more double quotes than single quotes.
	PreferDouble = "preferDouble"

	#: Prefer using single quotes except in scenarios where the string contains more single quotes than double quotes.
	PreferSingle = "preferSingle"


class QuoteProps(str, Enum):
	"""
	Behaviour to use for quotes on property names.
	"""

	#: Remove unnecessary quotes around property names.
	AsNeeded = "asNeeded"

	#: Same as `AsNeeded`, but if one property requires quotes then quote them all.
	Consistent = "consistent"

	#: Preserve quotes around property names.
	Preserve = "preserve"


class JsxMultiLineParens(str, Enum):
	"""
	Whether to surround a JSX element or fragment with parentheses when it's the top JSX node and it spans multiple lines.
	"""

	#: Never wrap JSX with parentheses.
	Never = "never"

	#: Prefer wrapping with parentheses in most scenarios, except in function arguments and JSX attributes.
	Prefer = "prefer"

	#: Always wrap JSX with parentheses if it spans multiple lines.
	Always = "always"


class SemiColonOrComma(str, Enum):
	"""
	Whether to use semi-colons or commas.
	"""
	#: Use semi colons (default).
	SemiColon = "semiColon"
	#: Use commas.
	Comma = "comma"


class SortOrder(str, Enum):
	"""
	The kind of sort ordering to use.
	"""

	#: Maintains the current ordering.
	Maintain = "maintain"

	#: Alphabetically and case sensitive.
	CaseSensitive = "caseSensitive"

	#: Alphabetically and case insensitive.
	CaseInsensitive = "caseInsensitive"


class NamedTypeImportsExportsOrder(str, Enum):
	First = "first"
	Last = "last"
	_None = "none"


class NewLineKind(str, Enum):

	#: Decide which newline kind to use based on the last newline in the file.
	Auto = "auto"

	#: Use slash n new lines.
	LineFeed = "lf"

	#: Use slash r slash n new lines.
	CarriageReturnLineFeed = "crlf"
