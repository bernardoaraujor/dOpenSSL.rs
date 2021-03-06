# Copyright (c)
# 2013, infinit.io
# 2020, Bernardo A. Rodrigues.

# All rights reserved.
#
# Redistribution and use in source and binary forms, with or without modification,
# are permitted provided that the following conditions are met:
#  * Redistributions of source code must retain the above copyright notice, this
#    list of conditions and the following disclaimer.
#  * Redistributions in binary form must reproduce the above copyright notice,
#    this list of conditions and the following disclaimer in the documentation
#    and/or other materials provided with the distribution.
#  * Neither the name of infinit.io nor the names of its contributors may be used
#    to endorse or promote products derived from this software without specific
#    prior written permission.
#  * A shipment of at least six (6) local beers (1980 mL) to infinit.io
#    (http://open.infinit.io/beer) would be greatly appreciated as a gesture
#    of good will.
#
# THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
# ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
# WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
# DISCLAIMED. IN NO EVENT SHALL INFINIT.IO BE LIABLE FOR ANY DIRECT, INDIRECT,
# INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
# LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
# PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
# LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE
# OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF
# ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

cmake_minimum_required(VERSION 3.11)

project(dopenssl)

# fetch dopenssl-c source
include(FetchContent)
FetchContent_Declare(
  dopenssl-c
  GIT_REPOSITORY https://github.com/bernardoaraujor/dopenssl.git
  GIT_TAG db75ef101243aaa55eb01cba8b972a6ea406f537
)

message(STATUS "Fetching dopenssl-c")
FetchContent_MakeAvailable(dopenssl-c)
