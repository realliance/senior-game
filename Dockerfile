ARG EDITOR_IMAGE=quay.io/realliance/unity-editor
ARG EDITOR_TAG=2021.1.0b5

FROM $EDITOR_IMAGE:$EDITOR_TAG AS builder

WORKDIR /build/

ADD . .

RUN mkdir -p /root/.local/share/unity3d/Unity/ build && \
    cp Unity_v2021.x.ulf '/root/.local/share/unity3d/Unity/Unity_lic.ulf' && \
    unity-editor \
    -nographics \
    -batchmode \
    -projectPath '.' \
    -executeMethod Builder.BuildServer \
    -quit

RUN rm -rf Build/WizardConnect3_BackUpThisFolder_ButDontShipItWithYourGame

FROM mcr.microsoft.com/dotnet/runtime:5.0

WORKDIR /run

COPY --from=builder /build/Build /run

CMD ["./WizardConnect3", "-batchmode", "-nographics"]
