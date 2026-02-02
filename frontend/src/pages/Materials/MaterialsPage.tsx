import { MaterialsSection } from "@/common/organisms";
import { MOCK_MATERIALS } from "@/mocks";

const MaterialsPage = () => {
  return (
    <div className="container w-5/6 m-auto max-w-screen-2xl py-6">
      <MaterialsSection materials={MOCK_MATERIALS} />
    </div>
  );
};

export default MaterialsPage;
